use crate::UndecidedTile;
use crate::TileGrid;
use ::rand::distributions::WeightedIndex;
use ::rand::prelude::*;
use std::cmp::Ordering;
use std::hash::Hash;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct TileChoice {
    pub connections: [Connection; 4], // up right down left
    pub weight: i32,
    pub texture: &'static str,
    pub flipx: bool,
    pub flipy: bool,
    pub rot90: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Connection {
    OrangeOut,
    OrangeIn,
    OrangeFlat,
    PurpleOut,
    PurpleIn,
    PurpleFlat,
    GreenOut,
    GreenIn,
    GreenFlat,
    YellowOut,
    YellowIn,
    YellowFlat,
}

impl Connection {
    pub fn can_connect (con1: Connection, con2: Connection) -> bool {
        let outs = [Connection::OrangeOut, Connection::PurpleOut, Connection::GreenOut, Connection::YellowOut];
        let ins = [Connection::OrangeIn, Connection::PurpleIn, Connection::GreenIn, Connection::YellowIn];
        let flats = [Connection::OrangeFlat, Connection::PurpleFlat, Connection::GreenFlat, Connection::YellowFlat];
        if outs.contains(&con1) {
            return ins.contains(&con2);
        }
        else if ins.contains(&con1) {
            return outs.contains(&con2) || flats.contains(&con2);
        }
        else if flats.contains(&con1) {
            return flats.contains(&con2) || ins.contains(&con2);
        }
        else {
            return false;
        }
    }
}

impl UndecidedTile {
    pub fn new() -> Self {
        let mut possible_tiles = Vec::<TileChoice>::new();
        
        use Connection as c;
        //Orange
        let connections = [c::OrangeFlat, c::OrangeOut, c::OrangeIn, c::OrangeFlat];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "1", flipx: false, flipy: false, rot90: false});
        let connections = [c::OrangeFlat, c::OrangeFlat, c::OrangeOut, c::OrangeIn];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "1", flipx: false, flipy: false, rot90: true});
        let connections = [c::OrangeIn, c::OrangeFlat, c::OrangeFlat, c::OrangeOut];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "1", flipx: true, flipy: true, rot90: false});
        let connections = [c::OrangeOut, c::OrangeIn, c::OrangeFlat, c::OrangeFlat];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "1", flipx: true, flipy: true, rot90: true});

        // Purple
        let connections = [c::PurpleOut, c::PurpleOut, c::PurpleIn, c::PurpleFlat];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "2", flipx: false, flipy: false, rot90: false});
        let connections = [c::PurpleFlat, c::PurpleOut, c::PurpleOut, c::PurpleIn];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "2", flipx: false, flipy: false, rot90: true});
        let connections = [c::PurpleIn, c::PurpleFlat, c::PurpleOut, c::PurpleOut];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "2", flipx: true, flipy: true, rot90: false});
        let connections = [c::PurpleOut, c::PurpleIn, c::PurpleFlat, c::PurpleOut];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "2", flipx: true, flipy: true, rot90: true});

        // Green
        let connections = [c::GreenOut, c::GreenOut, c::GreenOut, c::GreenFlat];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "3", flipx: false, flipy: false, rot90: false});
        let connections = [c::GreenFlat, c::GreenOut, c::GreenOut, c::GreenOut];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "3", flipx: false, flipy: false, rot90: true});
        let connections = [c::GreenOut, c::GreenFlat, c::GreenOut, c::GreenOut];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "3", flipx: true, flipy: true, rot90: false});
        let connections = [c::GreenOut, c::GreenOut, c::GreenFlat, c::GreenOut];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "3", flipx: true, flipy: true, rot90: true});

        // Yellow
        let connections = [c::YellowIn, c::YellowIn, c::YellowFlat, c::YellowFlat];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "4", flipx: false, flipy: false, rot90: false});
        let connections = [c::YellowFlat, c::YellowIn, c::YellowIn, c::YellowFlat];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "4", flipx: false, flipy: false, rot90: true});
        let connections = [c::YellowFlat, c::YellowFlat, c::YellowIn, c::YellowIn];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "4", flipx: true, flipy: true, rot90: false});
        let connections = [c::YellowIn, c::YellowFlat, c::YellowFlat, c::YellowIn];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "4", flipx: true, flipy: true, rot90: true});

        Self {
            possible_tiles,
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
        let self_option = self.possible_tiles[dist.sample(&mut ::rand::thread_rng())];
        self.possible_tiles = vec![self_option];
    }
}

impl TileGrid {
    pub fn pick_index(&mut self) -> Option<(i32, i32)> {
        let mut candidate_indices = Vec::<(i32, i32)>::new();
        let mut least_seen = 100000;
        let mut weights = Vec::<i32>::new();
        let mut total_seen = 0;
        const RESTRICTED_WEIGHT: i32 = 1; //when this is high, it will prioritize tiles with the least options. Cannot be 0
        const FREE_WEIGHT: i32 = 000; //when this is high, it will prioritize tiles that don't have the least options
        // restricted_weight is good for when the ruleset is restrictive (such as "all tiles must have precisely 2 connections"), and for making large blocks
        // free_weight is good for when you want smaller, more scattered blocks
        if self.width * self.height <= 1000 {
            for i in 0..self.width {
                for j in 0..self.height {
                    let tile = &self.tilegrid[i as usize][j as usize];
                    if tile.possible_tiles.len() == 1 {
                        continue;
                    }
                    total_seen += 1;
    
                    match tile.possible_tiles.len().cmp(&least_seen) {
                        Ordering::Less => {
                            least_seen = tile.possible_tiles.len();
                            weights = vec![FREE_WEIGHT;total_seen-1];
                            weights.push(RESTRICTED_WEIGHT);
                        }
                        Ordering::Equal => {
                            weights.push(RESTRICTED_WEIGHT);
                        },
                        Ordering::Greater => {
                            weights.push(FREE_WEIGHT);
                        },
                    }
                    candidate_indices.push((i, j));
                }
            }
        }
        else {
            for i in 0..50 {
                let x = ::rand::thread_rng().gen_range(0..self.width);
                let y = ::rand::thread_rng().gen_range(0..self.height);
                let tile = &self.tilegrid[x as usize][y as usize];
                if tile.possible_tiles.len() == 1 {
                    continue;
                }
                total_seen += 1;
                match tile.possible_tiles.len().cmp(&least_seen) {
                    Ordering::Less => {
                        least_seen = tile.possible_tiles.len();
                        weights = vec![FREE_WEIGHT;total_seen-1];
                        weights.push(RESTRICTED_WEIGHT);
                    }
                    Ordering::Equal => {
                        weights.push(RESTRICTED_WEIGHT);
                    },
                    Ordering::Greater => {
                        weights.push(FREE_WEIGHT);
                    },
                }
                candidate_indices.push((x, y));
            }
        }

        if candidate_indices.is_empty() {
            return None;
        }
        else {
            let dist = WeightedIndex::new(&weights).unwrap();
            let (x_index, y_index) = candidate_indices[dist.sample(&mut ::rand::thread_rng())];
            return Some((x_index, y_index));
        }
    }
}