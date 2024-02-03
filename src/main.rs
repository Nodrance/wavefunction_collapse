use macroquad::prelude::*;
use ::rand::seq::SliceRandom;
mod renderers;
use renderers::whitegrid::draw_tilegrid as draw_whitegrid;
use renderers::triangles::draw_tilegrid as draw_tilegrid;
mod wavefunctions;
use wavefunctions::colored_wires::*;
// use wavefunctions::colored_wires::*;              
// use std::thread;
// use std::ops::Index;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[repr(u32)]
// enum TextureId {
//     U = 0,
//     D = 1,
//     UD = 2,
//     L = 3,
//     UL = 4,
//     DL = 5,
//     UDL = 6,
//     R = 7,
//     UR = 8,
//     DR = 9,
//     UDR = 10,
//     LR = 11,
//     ULR = 12,
//     DLR = 13,
//     UDLR = 14,
// }
// impl TextureId {
//     fn path(&self) -> &'static str {
//         match self {
//             Self::U => "assets/U.png",
//             Self::D => "assets/D.png",
//             Self::UD => "assets/UD.png",
//             Self::L => "assets/L.png",
//             Self::UL => "assets/UL.png",
//             Self::DL => "assets/DL.png",
//             Self::UDL => "assets/UDL.png",
//             Self::R => "assets/R.png",
//             Self::UR => "assets/UR.png",
//             Self::DR => "assets/DR.png",
//             Self::UDR => "assets/UDR.png",
//             Self::LR => "assets/LR.png",
//             Self::ULR => "assets/ULR.png",
//             Self::DLR => "assets/DLR.png",
//             Self::UDLR => "assets/UDLR.png",
//         }
//     }
// }
// struct TextureFiles(Vec<Texture2D>);

// impl Index<TextureId> for TextureFiles {
//     type Output = Texture2D;
//     fn index(&self, item: TextureId) -> &Self::Output {
//         &self.0[item as usize]
//     }
// }

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
        for _ in 0..height {
            let mut row = Vec::<UndecidedTile>::new();
            for _ in 0..width {
                row.push(UndecidedTile::new());
            }
            tilegrid.push(row);
        }
        let mut output = Self {
            tilegrid: tilegrid,
            width: width,
            height: height,
            tilewidth: tilewidth,
            tileheight: tileheight,
            marginx: marginx,
            marginy: marginy,
        };
        output.restrict_grid();
        return output;
    }

    fn expand_to (&mut self, width: i32, height: i32) {
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
        self.restrict_grid();
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
        self.restrict_grid();
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
                    tile.possible_tiles[tile_option_index]
                    .clone().connections[connection_direction];

                let mut can_connect = false;
                for connection in possible_connections.iter() {
                    if Connection::can_connect(*connection, self_connection) {
                        can_connect = true;
                    }
                }
                if !can_connect {
                    tile.possible_tiles.remove(tile_option_index);
                    assert!(tile.possible_tiles.len() > 0, "No possible tiles left at ({}, {}), Rules are likely too restrictive. Please try again.", x, y);
                    did_something = true;
                }
            }
        }
        return did_something;
    }

    fn restrict_and_propegate (&mut self, vec: Vec<(i32, i32)>) { // Will restrict all tile indexes in the vec and propegate changes
        let mut todo_indices = vec.clone();
        let mut index = 0;
        while index < todo_indices.len() {
            let (x, y) = todo_indices[index];
            index += 1;
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
        let mut index = 0;
        while index < todo_indices.len() {
            let (x, y) = todo_indices[index];
            index += 1;
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rendermode {
    Texture,
    Ribbon,
    Triangle,
}

#[macroquad::main("WavefunctionCollapse")]
async fn main() {
    // texture loading 

    // for textureindex in 0..15 {
    //     let thread = thread::spawn(|| async {
    //         let texturepath = 
    //         let texture = load_texture(texture.path()).await.unwrap();
    //     });
    //     let result = thread.join();
    // }

    let mut grid = TileGrid::new(30, 30, 20.0, 20.0, 20.0, 20.0);
    let mut autogenerate;
    let mut rendermode = Rendermode::Ribbon;

    for _ in 0..grid.width {
        let mut row = Vec::<UndecidedTile>::new();
        for _ in 0..grid.height {
            row.push(UndecidedTile::new());
        }
        grid.tilegrid.push(row);
    }
    grid.restrict_grid();

    loop {
        clear_background(GREEN);
        draw_whitegrid(&grid);

        if is_key_down(KeyCode::T) {rendermode = Rendermode::Texture;}
        if is_key_down(KeyCode::R) {rendermode = Rendermode::Ribbon;}
        if is_key_down(KeyCode::Y) {rendermode = Rendermode::Triangle;}

        if is_key_pressed(KeyCode::Up) {grid.shift(0, 1);}
        if is_key_pressed(KeyCode::Down) {grid.shift(0, -1);}
        if is_key_pressed(KeyCode::Left) {grid.shift(-1, 0);}
        if is_key_pressed(KeyCode::Right) {grid.shift(1, 0);}
        if is_key_pressed(KeyCode::W) {grid.expand_to(grid.width, grid.height-1);}
        if is_key_pressed(KeyCode::S) {grid.expand_to(grid.width, grid.height+1);}
        if is_key_pressed(KeyCode::A) {grid.expand_to(grid.width-1, grid.height);}
        if is_key_pressed(KeyCode::D) {grid.expand_to(grid.width+1, grid.height);}

        match rendermode {
            Rendermode::Texture => draw_tilegrid(&grid),
            Rendermode::Ribbon => renderers::wires::draw_tilegrid(&grid),
            Rendermode::Triangle => renderers::triangles::draw_tilegrid(&grid),
        }

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

// use std::thread;
// fn mainx(){
//     let mut worked = 0;
//     for _ in 0..1000 {
//         let thread = thread::spawn(|| {
//             main();
//         });
//         let result = thread.join();
//         if result.is_ok() {
//             worked += 1;
//         }
//     }
//     println!("Worked: {}", worked);
// }




