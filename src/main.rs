#![allow(clippy::needless_return)]

use macroquad::prelude::*;
use ::rand::seq::SliceRandom;
use std::collections::HashMap;

mod renderers;
use renderers::whitegrid::draw_tilegrid as draw_whitegrid;
use renderers::debug_grid_draw::draw_tilegrid as debug_draw_tilegrid;
use renderers::beach::load_textures_paths;
use renderers::beach::draw_tilegrid;
use renderers::beach::draw_tile_opt;

mod wavefunctions;
use wavefunctions::islands::*;

use std::cmp::max;
use std::cmp::min;

#[derive(Clone, PartialEq, Debug)]
struct UndecidedTile {
    possible_tiles: Vec<TileChoice>,
}

#[derive(Clone, Debug)]
/// Contains a 2d grid of tiles, functions relating to manipulating that grid, and functions and data relating to rendering that grid
struct TileGrid {
    /// The grid of tiles
    tilegrid: Vec<Vec<UndecidedTile>>,
    /// The width of the grid in tiles
    width: i32,
    /// The height of the grid in tiles
    height: i32,
    ///the texture which the tilegrid is rendered to
    tilegrid_texture: RenderTarget,
    /// The number of pixels wide each tile is
    lod_x: i32,
    /// The number of pixels tall each tile is
    lod_y: i32,
    /// whether or not to render a white grid over the tilegrid
    whitegrid: bool,
    /// Whether to render the tilegrid as a texture or as debug information
    rendermode: Rendermode,
    /// A hashmap of textures to use for rendering the tilegrid
    texturemap: HashMap<&'static str, Texture2D>,
}

/// Functions relating to the tiles and tilegrid
impl TileGrid {
    fn new (width: i32, height: i32, texturemap: HashMap<&'static str, Texture2D>) -> Self {
        let mut tilegrid = Vec::<Vec<UndecidedTile>>::new();
        for _ in 0..width {
            let mut col = Vec::<UndecidedTile>::new();
            for _ in 0..height {
                col.push(UndecidedTile::new());
            }
            tilegrid.push(col);
        }
        let mut output = Self {
            tilegrid,
            width,
            height,
            tilegrid_texture: render_target(10, 10),
            lod_x: 1,
            lod_y: 1,
            whitegrid: false,
            rendermode: Rendermode::Texture,
            texturemap,
        };
        output.restrict_grid();
        return output;
    }

    fn expand_to (&mut self, width: i32, height: i32) {
        let old_height = self.height;
        let old_width = self.width;
        if width < 1 || height < 1 {
            return;
        }
        while self.height < height {
            for col in self.tilegrid.iter_mut() {
                col.push(UndecidedTile::new());
            }
            self.height += 1;
        }
        while self.width < width {
            let mut col = Vec::<UndecidedTile>::new();
            for _ in 0..self.height {
                col.push(UndecidedTile::new());
            }
            self.tilegrid.push(col);
            self.width += 1;
        }
        if self.width > width {
            self.tilegrid.truncate(width as usize);
            self.width = width;
        }
        if self.height > height {
            for col in self.tilegrid.iter_mut() {
                col.truncate(height as usize);
            }
            self.height = height;
        }
        let mut to_propegate = Vec::<(i32, i32)>::new();
        for i in 0..old_width {
            for j in old_height..self.height {
                to_propegate.push((i, j));
            }
        }
        for i in old_width..self.width {
            for j in 0..self.height {
                to_propegate.push((i, j));
            }
        }
        self.restrict_and_propegate(to_propegate);
    }

    fn shift (&mut self, x: i32, y: i32) {
        let mut new_tilegrid = Vec::<Vec<UndecidedTile>>::new();
        for i in 0..self.width {
            let mut col = Vec::<UndecidedTile>::new();
            for j in 0..self.height {
                let new_x = i + x;
                let new_y = j + y;
                if new_x < 0 || new_x >= self.width || new_y < 0 || new_y >= self.height {
                    col.push(UndecidedTile::new());
                }
                else {
                    col.push(self.tilegrid[new_x as usize][new_y as usize].clone());
                }
            }
            new_tilegrid.push(col);
        }
        self.tilegrid = new_tilegrid;
        let top = max(0, -y);
        let bottom = min(self.height, self.height-y);
        let left = max(0, -x);
        let right = min(self.width, self.width-x);
        self.restrict_grid_edges(top, bottom, left, right);
    }

    fn restrict_tile (&mut self, x: i32, y: i32) -> bool { // returns true if a change was made
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return false;
        }
        let mut did_something = false;
        let mut tile = self.tilegrid[x as usize][y as usize].clone();
        for connection_direction in 0..4 {
            let neighbor_indices = match connection_direction {
                0 => (x,y-1),
                1 => (x,y+1),
                2 => (x-1,y),
                _ => (x+1,y),
            };
            let neighbor_connection_direction = match connection_direction {
                0 => 1,
                1 => 0,
                2 => 3,
                _ => 2,
            };
            if neighbor_indices.0 < 0 || neighbor_indices.0 >= self.width || neighbor_indices.1 < 0 || neighbor_indices.1 >= self.height {
                continue;
            }

            let neighbor_tile = self.tilegrid[neighbor_indices.0 as usize][neighbor_indices.1 as usize].clone();
            let mut possible_connections = Vec::<Connection>::new();
            for tile_option in neighbor_tile.possible_tiles.iter() {
                let connection = tile_option.connections[neighbor_connection_direction];
                if !possible_connections.contains(&connection) {
                    possible_connections.push(connection);
                }
            }

            for tile_option_index in (0..tile.possible_tiles.len()).rev() {
                let self_connection = 
                    tile.possible_tiles[tile_option_index].connections[connection_direction];

                let mut can_connect = false;
                for connection in possible_connections.iter() {
                    if Connection::can_connect(*connection, self_connection) {
                        can_connect = true;
                    }
                }
                if !can_connect {
                    tile.possible_tiles.remove(tile_option_index);
                    assert!(!tile.possible_tiles.is_empty(), "No possible tiles left at ({}, {}), Rules are likely too restrictive. Please try again.", x, y);
                    did_something = true;
                }
            }
        }
        if did_something {
            self.tilegrid[x as usize][y as usize] = tile;
        }
        return did_something;
    }

    fn restrict_and_propegate (&mut self, vec: Vec<(i32, i32)>) { // Will restrict all tile indexes in the vec and propegate changes
        let mut todo_indices = vec.clone();
        while let Some((x, y)) = todo_indices.pop() {
            if self.restrict_tile(x, y) {
                for i in 0..4 {
                    let neighbor_indices = match i {
                        0 => (x,y-1),
                        1 => (x,y+1),
                        2 => (x-1,y),
                        _ => (x+1,y),
                    };
                    if !todo_indices.contains(&neighbor_indices) {
                        todo_indices.push(neighbor_indices);
                    }
                }
            }
        }
    }

    fn collapse_and_propegate (&mut self, x: i32, y: i32) { // Will collapse the tile at the index and propegate changes

        self.tilegrid[x as usize][y as usize].collapse();
        let todo_indices = vec![(x, y), (x, y-1), (x, y+1), (x-1, y), (x+1, y)];
        self.restrict_and_propegate(todo_indices);
    }

    fn restrict_grid (&mut self) { // By the end of the function, there will be no cases where a tile has an invalid possibility
        let mut todo_indices = Vec::<(i32, i32)>::new();
        for i in 0..self.width {
            for j in 0..self.height {
                todo_indices.push((i, j));
            }
        }    
        self.restrict_and_propegate(todo_indices);
    }

    fn restrict_grid_edges(&mut self, top:i32, bottom:i32, left:i32, right:i32) { // Restricts the edges of the grid
        let mut todo_indices = Vec::<(i32, i32)>::new();
        for i in 0..self.width {
            for j in 0..bottom {
                todo_indices.push((i, j));
            }
            for j in top..self.height {
                todo_indices.push((i, j));
            }
        }
        for j in bottom..top {
            for i in 0..left {
                todo_indices.push((i, j));
            }
            for i in right..self.width {
                todo_indices.push((i, j));
            }
        }
        self.restrict_and_propegate(todo_indices);
    }

    fn unrestrict_grid (&mut self) { // Will remove all restrictions unless a tile has only one possible tile
        for i in 0..self.width {
            for j in 0..self.height {
                if self.tilegrid[i as usize][j as usize].possible_tiles.len() > 1 {
                    self.tilegrid[i as usize][j as usize] = UndecidedTile::new();
                }
            }
        }
        self.restrict_grid();
    }

}

/// Functions relating to rendering and textures
impl TileGrid {

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rendermode {
    Texture,
    Debug,
}

#[macroquad::main("WavefunctionCollapse")]
async fn main() {              

    let mut framecount = 0;
    let texturemap = load_textures_paths(
        &["assets/islands/beach.png", "assets/islands/beach_water_corner.png", "assets/islands/beach_land_corner.png", "assets/islands/land.png", "assets/islands/water.png"], 
        &["beach", "beach_water_corner", "beach_land_corner", "land", "water"]
    ).await;
    let mut grid = TileGrid::new(50, 50, texturemap);
    let mut zoom_x = 1.0;
    let mut zoom_y = 1.0;
    const MARGIN_X: f32 = 10.0;
    const MARGIN_Y: f32 = 10.0;

    //Persist between loops
    //framecount: the number of frames that have passed since the program started, used to only render a subset of tiles
    //Grid: the grid of tiles, along with metadata about how it should be rendered
    //*Whitegrid: Whether or not to render a white grid over the tilegrid
    //*Rendermode: Whether to render the tilegrid as a texture or as debug information
    //*Texturemap: A hashmap of textures to use for rendering the tilegrid
    //*Tilegrid texture: The texture that the tilegrid is rendered to
    //* Old lod x: The last lod x value, used to determine if the texture needs to be rerendered
    //* Old lod y: The last lod y value, used to determine if the texture needs to be rerendered
    //*Zoom x: The zoom level in the x direction, 3.0 is 32*3.0 pixels per tile
    //*Zoom y: The zoom level in the y direction, 5.0 is 32*5.0 pixels per tile
    //-Margin x: Used for the left margin, constant
    //-Margin y: Used for the top margin, constant

    //Local to each loop
    //Effective tilewidth: The width of a tile in pixels, taking zoom into account
    //Effective tileheight: The height of a tile in pixels, taking zoom into account
    //*Lod x: The number of pixels wide each tile is in the texture
    //*Lod y: The number of pixels tall each tile is in the texture
    //Width up: The next power of two up from the width
    //Height up: The next power of two up from the height
    //Width a: The width the texture represents (not accounting for screen limits)
    //Height a: The height the texture represents (not accounting for screen limits)
    //Width b: The width the texture represents (accounting for screen limits)
    //Height b: The height the texture represents (accounting for screen limits)
    //Width: The width the texture represents (unified)
    //Height: The height the texture represents (unified)



    loop {
        // Zoom
        {
            if is_key_down(KeyCode::Equal) && zoom_x < 30.0 && zoom_y < 30.0 {
                zoom_x *= 1.01;
                zoom_y *= 1.01;
            }
            if is_key_down(KeyCode::Minus) && zoom_x > 0.1 && zoom_y > 0.1 {
                zoom_x *= 0.99;
                zoom_y *= 0.99;
            }
        }
        let effective_tilewidth = 32.0 * zoom_x;
        let effective_tileheight = 32.0 * zoom_y;

        // Movement
        {
            if is_key_pressed(KeyCode::Up) {grid.shift(0, 1);}
            if is_key_pressed(KeyCode::Down) {grid.shift(0, -1);}
            if is_key_pressed(KeyCode::Left) {grid.shift(1, 0);}
            if is_key_pressed(KeyCode::Right) {grid.shift(-1, 0);}
            if is_key_pressed(KeyCode::W) {grid.expand_to(grid.width, grid.height-1);}
            if is_key_pressed(KeyCode::S) {grid.expand_to(grid.width, grid.height+1);}
            if is_key_pressed(KeyCode::A) {grid.expand_to(grid.width-1, grid.height);}
            if is_key_pressed(KeyCode::D) {grid.expand_to(grid.width+1, grid.height);}
        }

        // Mouse positioning
        let mouse_x_pos = mouse_position().0;
        let mouse_y_pos = mouse_position().1;
        let mut mouse_x = ((mouse_x_pos - MARGIN_X) / effective_tilewidth) as i32;
        let mut mouse_y = ((mouse_y_pos - MARGIN_Y) / effective_tileheight) as i32;
        if mouse_y < 0 {mouse_y = 0;}
        if mouse_y >= grid.height {mouse_y = grid.height-1;}
        if mouse_x < 0 {mouse_x = 0;}
        if mouse_x >= grid.width {mouse_x = grid.width-1;}

        // Mouse collapsing
        if is_mouse_button_down(MouseButton::Left) {
            grid.collapse_and_propegate(mouse_x, mouse_y);
        }

        // Forced collapsing
        {
            let mut num = 0;
            if is_key_pressed(KeyCode::Key1) {num = 1;}
            if is_key_pressed(KeyCode::Key2) {num = 2;}
            if is_key_pressed(KeyCode::Key3) {num = 3;}
            if is_key_pressed(KeyCode::Key4) {num = 4;}
            if is_key_pressed(KeyCode::Key5) {num = 5;}
            if is_key_pressed(KeyCode::Key6) {num = 6;}
            if is_key_pressed(KeyCode::Key7) {num = 7;}
            if is_key_pressed(KeyCode::Key8) {num = 8;}
            if is_key_pressed(KeyCode::Key9) {num = 9;}
            if is_key_pressed(KeyCode::Key0) {num = 10;}

            if num != 0 {
                if is_key_down(KeyCode::LeftShift) {num+=10;}
                if is_key_down(KeyCode::LeftAlt) {num+=20;}
                if is_key_down(KeyCode::LeftControl) {num+=40;}
                let tile = grid.tilegrid[mouse_x as usize][mouse_y as usize].clone();
                if num as usize <= tile.possible_tiles.len() {
                    let tileopt = tile.possible_tiles[num as usize - 1];
                    grid.tilegrid[mouse_x as usize][mouse_y as usize].possible_tiles = vec![tileopt];
                    grid.restrict_and_propegate(vec![(mouse_x, mouse_y), (mouse_x, mouse_y-1), (mouse_x, mouse_y+1), (mouse_x-1, mouse_y), (mouse_x+1, mouse_y)])
                }
            }
        }
        
        //Auto collapsing
        if is_key_down(KeyCode::Space) {
            for _ in 0..100 {// warp factor
                let x;
                let y;
                let indices = TileGrid::pick_index(&mut grid);
                if indices.is_none() {
                    continue;
                }
                (x, y) = indices.unwrap();
                grid.collapse_and_propegate(x, y)
            }
        }

        // Mouse Ungeneration
        if is_mouse_button_down(MouseButton::Right) {
            grid.tilegrid[mouse_x as usize][mouse_y as usize] = UndecidedTile::new();
            grid.unrestrict_grid();
        }

        //Render
        if is_key_down(KeyCode::I) {grid.rendermode = Rendermode::Texture;}
        if is_key_down(KeyCode::O) {grid.rendermode = Rendermode::Debug;}

        // All rendering code
        let lod_x = 2 << ((zoom_x*32.0_f32).log2() as i32);
        let lod_y = 2 << ((zoom_y*32.0_f32).log2() as i32);
        let width_up = 2 << (grid.width.ilog2());
        let height_up = 2 << (grid.height.ilog2());
        let texture_width = min(width_up * lod_x, screen_width() as i32 * 2);
        let texture_height = min(height_up * lod_y, screen_height() as i32 * 2);
        if lod_x != grid.lod_x || lod_y != grid.lod_y || texture_width != grid.tilegrid_texture.texture.width() as i32 || texture_height != grid.tilegrid_texture.texture.height() as i32 {
            let mut old_texture_width = grid.tilegrid_texture.texture.width();
            let mut old_texture_height = grid.tilegrid_texture.texture.height();
            let tilegrid_texture = render_target(texture_width as u32, texture_height as u32);
            tilegrid_texture.texture.set_filter(FilterMode::Nearest);
            set_camera(&Camera2D {
                render_target: Some(tilegrid_texture.clone()),
                .. Camera2D::from_display_rect(Rect::new(0.0, 0.0, tilegrid_texture.texture.width(), tilegrid_texture.texture.height()))
            });
            if lod_x > grid.lod_x || lod_y > grid.lod_y {
                old_texture_height *= 2.0;
                old_texture_width *= 2.0;
            }
            if lod_x < grid.lod_x || lod_y < grid.lod_y {
                old_texture_height /= 2.0;
                old_texture_width /= 2.0;
            }
            draw_texture_ex(
                &grid.tilegrid_texture.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(old_texture_width as f32, old_texture_height as f32)),
                    flip_y: true,
                    ..Default::default()
                },
            );
            println!("lod_x: {}, lod_y: {}, texture_width: {}, texture_height: {}", lod_x, lod_y, texture_width, texture_height);
            grid.lod_x = lod_x;
            grid.lod_y = lod_y;
        }
        
        set_camera(&Camera2D {
            render_target: Some(grid.tilegrid_texture.clone()),
            .. Camera2D::from_display_rect(Rect::new(0.0, 0.0, grid.tilegrid_texture.texture.width(), grid.tilegrid_texture.texture.height()))
        });


        framecount += 1;
        match grid.rendermode {
            Rendermode::Texture => draw_tilegrid(&grid, &grid.texturemap, framecount, lod_x as f32, lod_y as f32),
            _ => debug_draw_tilegrid(&grid),
        }

        //Grid
        if is_key_pressed(KeyCode::P) {grid.whitegrid = !grid.whitegrid;}
        if grid.whitegrid {
            draw_whitegrid(&grid);
        }
        let width_a = 32.0*width_up as f32*zoom_x;
        let height_a = 32.0*height_up as f32*zoom_y;
        let width_b = (grid.tilegrid_texture.texture.width()/lod_x as f32)*32.0*zoom_x;
        let height_b = (grid.tilegrid_texture.texture.height()/lod_y as f32)*32.0*zoom_y;
        let width = if grid.tilegrid_texture.texture.width() == screen_width() * 2.0 {width_b} else {width_a};
        let height = if grid.tilegrid_texture.texture.height() == screen_height() * 2.0 {height_b} else {height_a};
        set_default_camera();
        draw_texture_ex(
            &grid.tilegrid_texture.texture,
            MARGIN_X,
            MARGIN_Y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                flip_y: true,
                ..Default::default()
            },
        );

        // Mouse hovering
        {
            let tiles = grid.tilegrid[mouse_x as usize][mouse_y as usize].clone().possible_tiles;
            if tiles.len() > 1 {
                let x_spacing = effective_tilewidth * 1.1;
                let width = (tiles.len() as f32 + 0.35) * x_spacing;
                draw_rectangle(0.0, screen_height()-effective_tileheight*1.9, width, effective_tileheight*3.0, BLACK);
                for (i, tile) in tiles.iter().enumerate() {
                    draw_tile_opt( (i as f32 + 0.2) * x_spacing, screen_height()-effective_tileheight*1.3, effective_tilewidth, effective_tileheight, tile, &grid.texturemap);
                    let text_x = if i+1 < 10 {(i as f32 + 0.65) * x_spacing - effective_tileheight*0.15} else {(i as f32 + 0.65)* x_spacing - effective_tileheight*0.35};
                    draw_text(&format!("{}", i+1), text_x, screen_height()-effective_tileheight*1.4, effective_tileheight*0.7, WHITE);
                    if i%10 == 0 && i != 0 {
                        draw_line((i as f32 + 0.15) * x_spacing, screen_height()-effective_tileheight*1.9, (i as f32 + 0.15) * x_spacing, screen_height()-effective_tileheight*0.3, effective_tilewidth*0.1, WHITE);
                    }
                }
            }
            //Draw an outline around the selected tile
            draw_rectangle_lines(mouse_x as f32 * effective_tilewidth + MARGIN_X, mouse_y as f32 * effective_tileheight + MARGIN_Y, effective_tilewidth, effective_tileheight, effective_tilewidth*0.15, WHITE);
        }

        

        next_frame().await;
    }
}
