use crate::UndecidedTile;
use crate::TileGrid;
use ::rand::distributions::WeightedIndex;
use ::rand::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TileChoice {
    pub connections: [Connection; 4], // up down left right
    pub weight: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Connection {
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    White,
}

impl Connection {
    pub fn can_connect (con1: Connection, con2: Connection) -> bool {
        match con1 {
            Connection::Black => con2 == Connection::Black,
            Connection::Red => con2 == Connection::Red || con2 == Connection::Yellow || con2 == Connection::White,
            Connection::Green => con2 == Connection::Green || con2 == Connection::Yellow || con2 == Connection::White,
            Connection::Blue => con2 == Connection::Blue || con2 == Connection::Yellow || con2 == Connection::White,
            Connection::Yellow => con2 == Connection::Yellow || con2 == Connection::White,
            Connection::White => con2 == Connection::White,
        }
    }
}

impl UndecidedTile {
    pub fn new() -> Self {
        let mut possible_tiles = Vec::<TileChoice>::new();
        for connection in [Connection::Red, Connection::Blue, Connection::Green, Connection::Yellow, Connection::White].iter() {
            for i in 0..16 {
                let mut new_tile = TileChoice {connections: [Connection::Black; 4], weight: 1};
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
                if conns != 2 && conns != 0 {
                    continue;
                }
                if i == 0 {
                    new_tile.weight = 100;
                }
                else if i == 12 || i == 3 {
                    new_tile.weight = 10;
                }
                else {
                    new_tile.weight = 2;
                }
                // Deterministic Colors
                // let color = match connection {
                //     Connection::Black => BLACK,
                //     Connection::Red => RED,
                //     Connection::Blue => BLUE,
                //     Connection::Green => GREEN,
                //     Connection::Yellow => YELLOW,
                //     Connection::White => WHITE,
                // };
                // Random Colors
                // let colors = vec![RED, BLUE, GREEN, YELLOW, WHITE];
                // let color = colors.choose(&mut ::rand::thread_rng()).unwrap().clone();
                // new_tile.color = color;
                // if connection == &Connection::Red {
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
    pub fn collapse (&mut self) {
        if self.possible_tiles.len() == 1 {
            return;
        }
        let mut weights = Vec::<i32>::new();
        for self_option in self.possible_tiles.iter() {
            weights.push(self_option.weight);
        }
        let dist = WeightedIndex::new(&weights).unwrap();
        let self_option = self.possible_tiles[dist.sample(&mut ::rand::thread_rng())].clone();
        self.possible_tiles = vec![self_option];
    }
}

impl TileGrid {
    pub fn pick_index(&mut self) -> Option<(i32, i32)> {
        let mut candidate_indices = Vec::<(i32, i32)>::new();
        let mut least_seen = 100000;
        let mut weights = Vec::<i32>::new();
        let mut total_seen = 0;
        const LARGE_WEIGHT: i32 = 1;
        const SMALL_WEIGHT: i32 = 3;
        for i in 0..self.width {
            for j in 0..self.height {
                let tile = &self.tilegrid[i as usize][j as usize];
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
            return None;
        }
        else {
            let dist = WeightedIndex::new(&weights).unwrap();
            let (x_index, y_index) = candidate_indices[dist.sample(&mut ::rand::thread_rng())];
            return Some((x_index, y_index));
        }
    }
}