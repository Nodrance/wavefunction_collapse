#![allow(clippy::needless_return)]

use macroquad::prelude::*;
use ::rand::seq::SliceRandom;

mod renderers;
use renderers::whitegrid::draw_tilegrid as draw_whitegrid;
use renderers::debug_grid_draw::draw_tilegrid as debug_draw_tilegrid;
use renderers::beach::load_textures_paths;
use renderers::beach::draw_tilegrid;

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

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rendermode {
    Wire,
    Triangle,
    Texture,
    Debug,
}

#[macroquad::main("WavefunctionCollapse")]
async fn main() {                

    let mut grid = TileGrid::new(30, 15, 20.0, 20.0, 20.0, 20.0);
    let mut autogenerate;
    let mut whitegrid = false;
    let mut rendermode = Rendermode::Wire;
    let texturemap = load_textures_paths(
        &["assets/islands/beach.png", "assets/islands/beach_water_corner.png", "assets/islands/beach_land_corner.png", "assets/islands/land.png", "assets/islands/water.png"], 
        &["beach", "beach_water_corner", "beach_land_corner", "land", "water"]
    ).await;

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::R) {rendermode = Rendermode::Wire;}
        if is_key_down(KeyCode::T) {rendermode = Rendermode::Triangle;}
        if is_key_down(KeyCode::Y) {rendermode = Rendermode::Texture;}
        if is_key_down(KeyCode::U) {rendermode = Rendermode::Debug;}

        match rendermode {
            // Rendermode::Wire => draw_wiregrid(&grid),
            // Rendermode::Triangle => draw_trianglegrid(&grid),
            Rendermode::Texture => draw_tilegrid(&grid, &texturemap),
            _ => debug_draw_tilegrid(&grid),
        }

        if is_key_pressed(KeyCode::G) {whitegrid = !whitegrid;}
        if whitegrid {
            draw_whitegrid(&grid);
        }

        if is_key_pressed(KeyCode::Up) {grid.shift(0, 1);}
        if is_key_pressed(KeyCode::Down) {grid.shift(0, -1);}
        if is_key_pressed(KeyCode::Left) {grid.shift(1, 0);}
        if is_key_pressed(KeyCode::Right) {grid.shift(-1, 0);}
        if is_key_pressed(KeyCode::W) {grid.expand_to(grid.width, grid.height-1);}
        if is_key_pressed(KeyCode::S) {grid.expand_to(grid.width, grid.height+1);}
        if is_key_pressed(KeyCode::A) {grid.expand_to(grid.width-1, grid.height);}
        if is_key_pressed(KeyCode::D) {grid.expand_to(grid.width+1, grid.height);}

        autogenerate = is_key_down(KeyCode::Space);

        // pick a random undecided tile
        for _ in 0..1 {// warp factor
            let x;
            let y;
            if autogenerate {
                let indices = TileGrid::pick_index(&mut grid);
                if indices.is_none() {
                    continue;
                }
                (x, y) = indices.unwrap();
            }
            else if is_mouse_button_down(MouseButton::Left) {
                let mouse_x_pos = mouse_position().0;
                let mouse_y_pos = mouse_position().1;
                x = ((mouse_x_pos - grid.marginx) / grid.tilewidth) as i32;
                y = ((mouse_y_pos - grid.marginy) / grid.tileheight) as i32;
                if y < 0 || y >= grid.height || x < 0 || x >= grid.width {
                    continue;
                }
            }
            else {continue;}
            grid.tilegrid[x as usize][y as usize].collapse();
            let to_propegate = vec![(x, y), (x, y-1), (x, y+1), (x-1, y), (x+1, y)];
            grid.restrict_and_propegate(to_propegate);
        }
        next_frame().await;
    }
}
