#![allow(clippy::needless_return)]

use macroquad::prelude::*;
use ::rand::seq::SliceRandom;

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

#[derive(Clone, PartialEq, Debug)]
struct TileGrid {
    tilegrid: Vec<Vec<UndecidedTile>>,
    width: i32,
    height: i32,
    tilewidth: f32,
    tileheight: f32,
    marginx: f32,
    marginy: f32,
}

impl TileGrid {
    fn new (width: i32, height: i32, tilewidth: f32, tileheight: f32, marginx: f32, marginy: f32) -> Self {
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
            tilewidth,
            tileheight,
            marginx,
            marginy,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rendermode {
    Texture,
    Debug,
}
#[macroquad::main("WavefunctionCollapse")]
async fn main() {                

    let mut framecount = 0;
    let mut grid = TileGrid::new(5, 5, 20.0, 20.0, 0.0, 0.0);
    let mut whitegrid = false;
    let mut rendermode = Rendermode::Texture;
    let texturemap = load_textures_paths(
        &["assets/islands/beach.png", "assets/islands/beach_water_corner.png", "assets/islands/beach_land_corner.png", "assets/islands/land.png", "assets/islands/water.png"], 
        &["beach", "beach_water_corner", "beach_land_corner", "land", "water"]
    ).await;
    let mut tilegrid_texture = render_target(1, 1);
    let mut old_lod_x = 0;
    let mut old_lod_y = 0;

    loop {

        // Zoom
        {
            if is_key_down(KeyCode::Equal) {grid.tilewidth *= 1.01; grid.tileheight *= 1.01; grid.marginx *= 1.01; grid.marginy *= 1.01;}
            if is_key_down(KeyCode::Minus) {grid.tilewidth *= 0.99; grid.tileheight *= 0.99; grid.marginx *= 0.99; grid.marginy *= 0.99;}
        }

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
        let mut mouse_x = ((mouse_x_pos - grid.marginx) / grid.tilewidth) as i32;
        let mut mouse_y = ((mouse_y_pos - grid.marginy) / grid.tileheight) as i32;
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
        if is_key_down(KeyCode::I) {rendermode = Rendermode::Texture;}
        if is_key_down(KeyCode::O) {rendermode = Rendermode::Debug;}

        /*
        There are screenwidth/tilewidth * screenheight/tileheight tiles on the screen
        Each tile needs at least screenwidth/gridwidth pixels of width and screenheight/gridheight pixels of height
        The texture needs to be screenwidth/gridwidth * screenwidth/tilewidth pixels of width and screenheight/gridheight * screenheight/tileheight pixels of height
        */
        let tile_pixels_x = screen_width()/(grid.width as f32);
        let tile_pixels_y = screen_height()/(grid.height as f32);
        let lod_x = tile_pixels_x.log2() as i32;
        let lod_y = tile_pixels_y.log2() as i32;
        if lod_x != old_lod_x || lod_y != old_lod_y {
            let old_tilegrid_texture = tilegrid_texture.clone();
            tilegrid_texture = render_target((2^lod_x) as u32, (2^lod_y) as u32);
            tilegrid_texture.texture.set_filter(FilterMode::Nearest);
            set_camera(&Camera2D {
                zoom: vec2(1.0, 1.0),
                target: vec2(0.0, 0.0),
                render_target: Some(tilegrid_texture.clone()),
                ..Default::default()
            });
            draw_texture_ex(
                &old_tilegrid_texture.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2((2^old_lod_x) as f32, (2^old_lod_y) as f32)),
                    ..Default::default()
                },
            );
            old_lod_x = lod_x;
            old_lod_y = lod_y;
        }
        




        set_camera(&Camera2D {
            zoom: vec2(1.0, 1.0),
            target: vec2(0.0, 0.0),
            render_target: Some(tilegrid_texture.clone()),
            ..Default::default()
        });
        // All rendering code
        framecount += 1;
        match rendermode {
            Rendermode::Texture => draw_tilegrid(&grid, &texturemap, framecount, (2^lod_x) as f32, (2^lod_y) as f32),
            _ => debug_draw_tilegrid(&grid),
        }

        //Grid
        if is_key_pressed(KeyCode::P) {whitegrid = !whitegrid;}
        if whitegrid {
            draw_whitegrid(&grid);
        }

        set_default_camera();
        draw_texture_ex(
            &tilegrid_texture.texture,
            // texturemap.get("land").unwrap(),
            grid.marginx,
            grid.marginy,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(grid.width as f32 * grid.tilewidth, grid.height as f32 * grid.tileheight)),
                ..Default::default()
            },
        );

        // Mouse hovering
        {
            let tiles = grid.tilegrid[mouse_x as usize][mouse_y as usize].clone().possible_tiles;
            if tiles.len() > 1 {
                let x_spacing = grid.tilewidth * 1.1;
                let width = (tiles.len() as f32 + 0.35) * x_spacing;
                draw_rectangle(0.0, screen_height()-grid.tileheight*1.9, width, grid.tileheight*3.0, BLACK);
                for (i, tile) in tiles.iter().enumerate() {
                    draw_tile_opt( (i as f32 + 0.2) * x_spacing, screen_height()-grid.tileheight*1.3, grid.tilewidth, grid.tileheight, tile, &texturemap);
                    let text_x = if i+1 < 10 {(i as f32 + 0.65) * x_spacing - grid.tileheight*0.15} else {(i as f32 + 0.65)* x_spacing - grid.tileheight*0.35};
                    draw_text(&format!("{}", i+1), text_x, screen_height()-grid.tileheight*1.4, grid.tileheight*0.7, WHITE);
                    if i%10 == 0 && i != 0 {
                        draw_line((i as f32 + 0.15) * x_spacing, screen_height()-grid.tileheight*1.9, (i as f32 + 0.15) * x_spacing, screen_height()-grid.tileheight*0.3, grid.tilewidth*0.1, WHITE);
                    }
                }
            }
            //Draw an outline around the selected tile
            draw_rectangle_lines(mouse_x as f32 * grid.tilewidth + grid.marginx, mouse_y as f32 * grid.tileheight + grid.marginy, grid.tilewidth, grid.tileheight, grid.tilewidth*0.15, WHITE);
        }

        

        next_frame().await;
    }
}
