use crate::UndecidedTile;
use crate::TileGrid;
use ::rand::distributions::WeightedIndex;
use ::rand::prelude::*;
use std::cmp::Ordering;
use std::hash::Hash;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct TileChoice {
    pub connections: [Connection; 4], // up down left right
    pub weight: i32,
    pub texture: &'static str,
    pub flipx: bool,
    pub flipy: bool,
    pub rot90: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Connection {
    Land,
    Water,
    BeachCW,
    BeachCCW,
}

impl Connection {
    pub fn can_connect (con1: Connection, con2: Connection) -> bool {
        if con1 == Connection::Land || con1 == Connection::Water {
            return con1 == con2;
        }
        else if con1 == Connection::BeachCW {
            return con2 == Connection::BeachCCW
        }
        else if con1 == Connection::BeachCCW {
            return con2 == Connection::BeachCW
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
        const BEACH_WEIGHT: i32 = 10;
        const BEACH_WATER_WEIGHT: i32 = 1;
        const BEACH_LAND_WEIGHT: i32 = 1;
        const LAND_WEIGHT: i32 = 30;
        const WATER_WEIGHT: i32 = 50;

        // Straight Beaches
        let mut cons = [c::Land, c::Water, c::BeachCW, c::BeachCCW];
        possible_tiles.push(TileChoice {connections: cons, weight: BEACH_WEIGHT, texture: "beach", flipx: false, flipy: false, rot90: false});
        cons = [c::BeachCW, c::BeachCCW, c::Water, c::Land];
        possible_tiles.push(TileChoice {connections: cons, weight: BEACH_WEIGHT, texture: "beach", flipx: false, flipy: false, rot90: true});
        cons = [c::Water, c::Land, c::BeachCCW, c::BeachCW];
        possible_tiles.push(TileChoice {connections: cons, weight: BEACH_WEIGHT, texture: "beach", flipx: true, flipy: true, rot90: false});
        cons = [c::BeachCCW, c::BeachCW, c::Land, c::Water];
        possible_tiles.push(TileChoice {connections: cons, weight: BEACH_WEIGHT, texture: "beach", flipx: true, flipy: true, rot90: true});

        // Watery Corners
        cons = [c::BeachCCW, c::Water, c::BeachCW, c::Water];
        possible_tiles.push(TileChoice {connections: cons, weight: BEACH_WATER_WEIGHT, texture: "beach_water_corner", flipx: false, flipy: false, rot90: false});
        cons = [c::BeachCW, c::Water, c::Water, c::BeachCCW];
        possible_tiles.push(TileChoice {connections: cons, weight: BEACH_WATER_WEIGHT, texture: "beach_water_corner", flipx: false, flipy: false, rot90: true});
        cons = [c::Water, c::BeachCCW, c::Water, c::BeachCW];
        possible_tiles.push(TileChoice {connections: cons, weight: BEACH_WATER_WEIGHT, texture: "beach_water_corner", flipx: true, flipy: true, rot90: false});
        cons = [c::Water, c::BeachCW, c::BeachCCW, c::Water];
        possible_tiles.push(TileChoice {connections: cons, weight: BEACH_WATER_WEIGHT, texture: "beach_water_corner", flipx: true, flipy: true, rot90: true});

        // Land Corners
        
        cons = [c::Land, c::BeachCCW, c::BeachCW, c::Land];
        possible_tiles.push(TileChoice {connections: cons, weight: BEACH_LAND_WEIGHT, texture: "beach_land_corner", flipx: false, flipy: false, rot90: false});
        cons = [c::BeachCW, c::Land, c::BeachCCW, c::Land];
        possible_tiles.push(TileChoice {connections: cons, weight: BEACH_LAND_WEIGHT, texture: "beach_land_corner", flipx: false, flipy: false, rot90: true});
        cons = [c::BeachCCW, c::Land, c::Land, c::BeachCW];
        possible_tiles.push(TileChoice {connections: cons, weight: BEACH_LAND_WEIGHT, texture: "beach_land_corner", flipx: true, flipy: true, rot90: false});
        cons = [c::Land, c::BeachCW, c::Land, c::BeachCCW];
        possible_tiles.push(TileChoice {connections: cons, weight: BEACH_LAND_WEIGHT, texture: "beach_land_corner", flipx: true, flipy: true, rot90: true});

        // Land and Water
        cons = [c::Land, c::Land, c::Land, c::Land];
        possible_tiles.push(TileChoice {connections: cons, weight: LAND_WEIGHT, texture: "land", flipx: false, flipy: false, rot90: false});
        cons = [c::Water, c::Water, c::Water, c::Water];
        possible_tiles.push(TileChoice {connections: cons, weight: WATER_WEIGHT, texture: "water", flipx: false, flipy: false, rot90: false});

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
        const FREE_WEIGHT: i32 = 1000; //when this is high, it will prioritize tiles that don't have the least options
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