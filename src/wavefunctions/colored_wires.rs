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
        return con1 == con2;
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
                if conns == 0 {
                    new_tile.weight = 1;
                }
                else
                if conns == 2 {
                    new_tile.weight = 1000;
                }
                else {
                    new_tile.weight = 10;
                }
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
        const RESTRICTED_WEIGHT: i32 = 1; //when this is high, it will prioritize tiles with the least options
        const FREE_WEIGHT: i32 = 3; //when this is high, it will prioritize tiles that don't have the least options
        // restricted_weight is good for when the ruleset is restrictive (such as "all tiles must have precisely 2 connections"), and for making large blocks
        // free_weight is good for when you want smaller, more scattered blocks
        // triangle renderer is recommended to see the difference
        for i in 0..self.width {
            for j in 0..self.height {
                let tile = &self.tilegrid[i as usize][j as usize];
                if tile.possible_tiles.len() == 1 {
                    continue;
                }
                total_seen += 1;
                if tile.possible_tiles.len() < least_seen {
                    least_seen = tile.possible_tiles.len();
                    weights = vec![FREE_WEIGHT;total_seen-1];
                    weights.push(RESTRICTED_WEIGHT);
                }
                else if tile.possible_tiles.len() == least_seen {
                    weights.push(RESTRICTED_WEIGHT);
                }
                else {
                    weights.push(FREE_WEIGHT);
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