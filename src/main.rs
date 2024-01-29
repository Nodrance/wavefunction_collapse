use macroquad::prelude::*;
use ::rand::seq::SliceRandom;
use ::rand::distributions::WeightedIndex;
use ::rand::prelude::*;
// use std::thread;
// use std::ops::Index;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Connection {
    Black,
    RedWire,
    BlueWire,
    GreenWire,
    YellowWire,
    WhiteWire,
}

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
struct TileChoice {
    connections: [Connection; 4],
    color: Color,
    texture: Option<Texture2D>,
    weight: i32,
}
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
    fn expand_to(&mut self, width: i32, height: i32) {
        while self.width < width {
            for row in self.tilegrid.iter_mut() {
                row.push(UndecidedTile::new());
            }
            self.width += 1;
        }
        while self.height < height {
            let mut row = Vec::<UndecidedTile>::new();
            for _ in 0..self.width {
                row.push(UndecidedTile::new());
            }
            self.tilegrid.push(row);
            self.height += 1;
        }
        if self.height > height {
            self.tilegrid.truncate(height as usize);
            self.height = height;
        }
        if self.width > width {
            for row in self.tilegrid.iter_mut() {
                row.truncate(width as usize);
            }
            self.width = width;
        }
    }
}

impl UndecidedTile {
    fn new() -> Self {
        let mut possible_tiles = Vec::<TileChoice>::new();
        for connection in [Connection::RedWire, Connection::BlueWire, /*Connection::GreenWire, Connection::YellowWire, Connection::WhiteWire*/].iter() {
            for i in 1..16 {
                let mut new_tile = TileChoice {connections: [Connection::Black; 4], color: BLACK, texture:None, weight: 1};
                let mut conns = 0;
                if i & 1 == 1 {
                    new_tile.connections[0] = connection.clone();
                    conns += 1;
                }
                if i & 2 == 2 {
                    new_tile.connections[1] = connection.clone();
                    conns += 1;
                }
                if i & 4 == 4 {
                    new_tile.connections[2] = connection.clone();
                    conns += 1;
                }
                if i & 8 == 8 {
                    new_tile.connections[3] = connection.clone();
                    conns += 1;
                } 
                if conns != 2 {
                    continue;
                }
                if i == 3 || i == 12 {
                    new_tile.weight = 2;
                }
                else {
                    new_tile.weight = 3;
                }
                // Deterministic Colors
                let color = match connection {
                    Connection::Black => BLACK,
                    Connection::RedWire => RED,
                    Connection::BlueWire => BLUE,
                    Connection::GreenWire => GREEN,
                    Connection::YellowWire => YELLOW,
                    Connection::WhiteWire => WHITE,
                };
                // Random Colors
                // let colors = vec![RED, BLUE, GREEN, YELLOW, WHITE];
                // let color = colors.choose(&mut ::rand::thread_rng()).unwrap().clone();
                new_tile.color = color;
                // if connection == &Connection::RedWire {
                //     let mut path = "./assets/".to_string();
                //     for i in 0..4 {
                //         if new_tile.connections[i] == Connection::Black {
                //             continue;
                //         }
                //         let char = match i {
                //             0 => "U",
                //             1 => "D",
                //             2 => "L",
                //             _ => "R",
                //         };
                //         path = path + char;
                //     }
                //     path = path + ".png";
                //     new_tile.texture = Some(load_texture(&path).await.unwrap());
                // }
                possible_tiles.push(new_tile);
            }
        }
        Self {
            possible_tiles: possible_tiles,
        }
    }
}


fn draw_grid (grid: &TileGrid) {
    for i in 0..grid.width {
        for j in 0..grid.height {
            let tile = &grid.tilegrid[i as usize][j as usize];
            for k in 0..4 {
                let tileopt = tile.possible_tiles.choose(&mut ::rand::thread_rng()).unwrap();
                let connection = tileopt.connections[k];

                if connection == Connection::Black {
                    continue;
                }
                let color = tileopt.color;

                let tx = (i as f32) * grid.tilewidth + grid.marginx;
                let ty = (j as f32) * grid.tileheight + grid.marginy;
                if tileopt.texture.is_some() {
                    let texture = tileopt.texture.as_ref().unwrap();
                    draw_texture_ex(texture, tx, ty, color, DrawTextureParams {
                        dest_size: Some(Vec2::new(grid.tilewidth, grid.tileheight)),
                        ..Default::default()
                    });
                }
                else if true { // ribbon style
                    let (x, y) = match k {
                        0 => (tx+(grid.tilewidth/3.0), ty),
                        1 => (tx+(grid.tilewidth/3.0), ty+(grid.tileheight/1.5)),
                        2 => (tx, ty+(grid.tileheight/3.0)),
                        3 => (tx+(grid.tilewidth/1.5), ty+(grid.tileheight/3.0)),
                        _ => (tx, ty),
                    };
                    draw_rectangle(x, y, grid.tilewidth/3.0, grid.tileheight/3.0, color);
                    draw_rectangle(tx+(grid.tilewidth/3.0), ty+(grid.tileheight/3.0), grid.tilewidth/3.0, grid.tileheight/3.0, color)
                }
                else { // triangle style
                    let tl = Vec2::new(tx+grid.tilewidth*0.1, ty+grid.tileheight*0.1);
                    let tr = Vec2::new(tx+grid.tilewidth*0.9, ty+grid.tileheight*0.1);
                    let bl = Vec2::new(tx+grid.tilewidth*0.1, ty+grid.tileheight*0.9);
                    let br = Vec2::new(tx+grid.tilewidth*0.9, ty+grid.tileheight*0.9);
                    let center = Vec2::new(tx+(grid.tilewidth/2.0), ty+(grid.tileheight/2.0));
                    let (v1, v2, v3) = match k {
                        0 => (tr, tl, center),
                        1 => (bl, br, center),
                        2 => (tl, bl, center),
                        _ => (br, tr, center),
                    };
                    draw_triangle(v1, v2, v3, color);
                }
            }
        }
    }
    // for i in 0..grid.height+1 {
    //     draw_line(grid.marginx, (i as f32) * grid.tileheight + grid.marginy,
    //               (grid.width as f32) * grid.tilewidth + grid.marginx, (i as f32) * grid.tileheight + grid.marginy,
    //               1.0, WHITE);
    // }
    // for i in 0..grid.width+1 {
    //     draw_line((i as f32) * grid.tilewidth + grid.marginx, grid.marginy,
    //               (i as f32) * grid.tilewidth + grid.marginx, (grid.height as f32) * grid.tileheight + grid.marginy, 
    //               1.0, WHITE);
    // }
}

fn pick_option (tile: UndecidedTile) -> UndecidedTile {
    let mut weights = Vec::<i32>::new();
    for tile_option in tile.possible_tiles.iter() {
        weights.push(tile_option.weight);
    }
    let dist = WeightedIndex::new(&weights).unwrap();
    let tile_option = tile.possible_tiles[dist.sample(&mut ::rand::thread_rng())].clone();
    let mut new_tile = tile.clone();
    new_tile.possible_tiles = vec![tile_option];
    new_tile
}

fn propegate_changes (grid: &mut TileGrid, x: i32, y: i32) {
    // collapse the tile at the given index, and propegate the collapse to the neighbors
    let mut todo_indices = vec![(x, y), (x-1, y), (x+1, y), (x, y-1), (x, y+1)];
    let mut index = 0;
    while index < todo_indices.len() {
        let (x, y) = todo_indices[index];
        index += 1;
        if x < 0 || x >= grid.height || y < 0 || y >= grid.width {
            continue;
        }
        let tile = grid.tilegrid[x as usize][y as usize].clone();
        for connection_direction in 0..4 {
            let mut did_something = false;
            let neighbor_direction_indices = match connection_direction {
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
            if neighbor_direction_indices.0 < 0 || neighbor_direction_indices.0 >= grid.height || neighbor_direction_indices.1 < 0 || neighbor_direction_indices.1 >= grid.width {
                continue;
            }
            let mut possible_connections = Vec::<Connection>::new();
            for tile_option in tile.possible_tiles.iter() {
                let connection = tile_option.connections[connection_direction];
                if !possible_connections.contains(&connection) {
                    possible_connections.push(connection);
                }
            }
            let neighbor_tile = grid.tilegrid[neighbor_direction_indices.0 as usize][neighbor_direction_indices.1 as usize].clone();
            let mut new_neighbor_tile = neighbor_tile.clone();
            for neighbor_tile_option_index in (0..neighbor_tile.possible_tiles.len()).rev() {
                let neighbor_tile_option = neighbor_tile.possible_tiles[neighbor_tile_option_index].clone();
                let neighbor_connection = neighbor_tile_option.connections[neighbor_connection_direction];
                if !possible_connections.contains(&neighbor_connection) {
                    new_neighbor_tile.possible_tiles.remove(neighbor_tile_option_index);
                    assert!(new_neighbor_tile.possible_tiles.len() > 0, "No possible tiles left at ({}, {}), Rules are likely too restrictive. Please try again.", neighbor_direction_indices.0, neighbor_direction_indices.1);
                    did_something = true;
                }
            }
            if did_something {
                grid.tilegrid[neighbor_direction_indices.0 as usize][neighbor_direction_indices.1 as usize] = new_neighbor_tile;
                todo_indices.push(neighbor_direction_indices.clone());
            }
        }
    }   
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
    // create a grid of new undecided tiles
    let mut grid = TileGrid{
        tilegrid: Vec::<Vec<UndecidedTile>>::new(),
        width: 2,
        height: 2,
        tilewidth: 5.0,
        tileheight: 5.0,
        marginx: 25.0,
        marginy: 25.0,
    };
    for _ in 0..grid.width {
        let mut row = Vec::<UndecidedTile>::new();
        for _ in 0..grid.height {
            row.push(UndecidedTile::new());
        }
        grid.tilegrid.push(row);
    }
    for x in 0..grid.width {
        for y in 0..grid.height {
            propegate_changes(&mut grid, x, y);
        }
    }

    loop {
        clear_background(BLACK);
        // draw a grid of black lines
        draw_grid(&grid);
        // pick a random undecided tile
        for _ in 0..10 {
        let mut candidate_indices = Vec::<(i32, i32)>::new();
        let mut least_seen = 100000;
        let mut weights = Vec::<i32>::new();
        let mut total_seen = 0;
        const LARGE_WEIGHT: i32 = 1;
        const SMALL_WEIGHT: i32 = 0;
        for i in 0..grid.height {
            for j in 0..grid.width {
                let tile = &grid.tilegrid[i as usize][j as usize];
                if tile.possible_tiles.len() == 1 {
                    continue;
                }
                total_seen += 1;
                if tile.possible_tiles.len() < least_seen {
                    least_seen = tile.possible_tiles.len();
                    weights = vec![SMALL_WEIGHT;total_seen-1];
                    weights.push(LARGE_WEIGHT);
                }
                else if tile.possible_tiles.len() == least_seen {
                    weights.push(LARGE_WEIGHT);
                }
                else {
                    weights.push(SMALL_WEIGHT);
                }
                candidate_indices.push((i, j));
            }
        }

        if candidate_indices.len() == 0 {
            if grid.width < 40 {
                let target_width = grid.width+1;
                let mut target_height = grid.height;
                if grid.height < 20 {
                    target_height += 1;
                }
                grid.expand_to(target_width, target_height);
                grid.tileheight = (screen_height() - grid.marginy*2.0) / (grid.height as f32);
                grid.tilewidth = (screen_width() - grid.marginx*2.0) / (grid.width as f32);
                for i in 0..grid.width {
                    for j in 0..grid.height {
                        propegate_changes(&mut grid, i, j);
                    }
                }
            }
            else {
                for row in 0..grid.tilegrid.len()-1 {
                    grid.tilegrid[row] = grid.tilegrid[row+1].clone();
                }
                grid.tilegrid.pop();
                let mut new_row = Vec::<UndecidedTile>::new();
                for _ in 0..grid.width {
                    new_row.push(UndecidedTile::new());
                }
                grid.tilegrid.push(new_row);
                for i in 0..grid.width {
                    let rowindex = grid.height-1;
                    propegate_changes(&mut grid, i, rowindex);
                }
            }
            
        }
        else {
            let dist = WeightedIndex::new(&weights).unwrap();
            let (x_index, y_index) = candidate_indices[dist.sample(&mut ::rand::thread_rng())];
            let mut tile = grid.tilegrid[x_index as usize][y_index as usize].clone();
            tile = pick_option(tile);
            grid.tilegrid[x_index as usize][y_index as usize] = tile;
            propegate_changes(&mut grid, x_index, y_index);
        }
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




