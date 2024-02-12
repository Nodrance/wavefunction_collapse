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
    DoorTop, // The side of the top of a door
    DoorBottom, // The side of the bottom of a door
    DoorRight, //The bottom of the right side of a door
    DoorLeft, // The bottom of the left side of a door
    WallLeft, // The left side of a wall, where the wall is on the right
    WallRight, // The right side of a wall, where the wall is on the left
    WallTop, // The top of a wall
    WallInner, // The interior of a wall
    Ground, // The ground
    Sky, // The sky
}

impl Connection {
    pub fn can_connect (con1: Connection, con2: Connection) -> bool {
        if con1 == con2 {
            return true;
        }
        // if con2 == Connection::Ground {
        //     return con1 == Connection::DoorRight || con1 == Connection::DoorLeft || con1 == Connection::Sky;
        // }
        // if con1 == Connection::Ground {
        //     return con2 == Connection::DoorRight || con2 == Connection::DoorLeft || con2 == Connection::Sky;
        // }
        return false;
    }
}

impl UndecidedTile {
    pub fn new() -> Self {
        let mut possible_tiles = Vec::<TileChoice>::new();

        use Connection as c;
        const DOOR_WEIGHT: i32 = 10;
        const WINDOW_WEIGHT: i32 = 2;
        const WALL_WEIGHT: i32 = 10;
        const EDGE_WEIGHT: i32 = 1;
        const TOP_WEIGHT: i32 = 100;
        const OUTER_CORNER_WEIGHT: i32 = 100;
        const INNER_CORNER_WEIGHT: i32 = 1;

        // Door
        let connections = [c::DoorLeft, c::DoorBottom, c::Ground, c::WallInner];
        possible_tiles.push(TileChoice {connections, weight: DOOR_WEIGHT, texture: "Door L", flipx: false, flipy: false, rot90: false});
        let connections = [c::DoorRight, c::WallInner, c::Ground, c::DoorBottom];
        possible_tiles.push(TileChoice {connections, weight: DOOR_WEIGHT, texture: "Door R", flipx: false, flipy: false, rot90: false});
        let connections = [c::WallInner, c::DoorTop, c::DoorLeft, c::WallInner];
        possible_tiles.push(TileChoice {connections, weight: DOOR_WEIGHT, texture: "Door TL", flipx: false, flipy: false, rot90: false});
        let connections = [c::WallInner, c::WallInner, c::DoorRight, c::DoorTop];
        possible_tiles.push(TileChoice {connections, weight: DOOR_WEIGHT, texture: "Door TR", flipx: false, flipy: false, rot90: false});

        // Wall and window
        let connections = [c::WallInner, c::WallInner, c::WallInner, c::WallInner];
        possible_tiles.push(TileChoice {connections, weight: WALL_WEIGHT, texture: "Wall 1", flipx: false, flipy: false, rot90: false});
        possible_tiles.push(TileChoice {connections, weight: WALL_WEIGHT, texture: "Wall 2", flipx: false, flipy: false, rot90: false});
        possible_tiles.push(TileChoice {connections, weight: WINDOW_WEIGHT, texture: "Window 1", flipx: false, flipy: false, rot90: false});
        possible_tiles.push(TileChoice {connections, weight: WINDOW_WEIGHT, texture: "Window 2", flipx: false, flipy: false, rot90: false});

        // Edge
        let connections = [c::WallLeft, c::WallInner, c::WallLeft, c::Sky];
        possible_tiles.push(TileChoice {connections, weight: EDGE_WEIGHT, texture: "Wall L1", flipx: false, flipy: false, rot90: false});
        possible_tiles.push(TileChoice {connections, weight: EDGE_WEIGHT, texture: "Wall L2", flipx: false, flipy: false, rot90: false});
        let connections = [c::WallRight, c::Sky, c::WallRight, c::WallInner];
        possible_tiles.push(TileChoice {connections, weight: EDGE_WEIGHT, texture: "Wall R1", flipx: false, flipy: false, rot90: false});
        possible_tiles.push(TileChoice {connections, weight: EDGE_WEIGHT, texture: "Wall R2", flipx: false, flipy: false, rot90: false});
        let connections = [c::Sky, c::WallTop, c::WallInner, c::WallTop];
        possible_tiles.push(TileChoice {connections, weight: TOP_WEIGHT, texture: "Wall T", flipx: false, flipy: false, rot90: false});
        
        // Corner
        let connections = [c::Sky, c::WallTop, c::WallLeft, c::Sky];
        possible_tiles.push(TileChoice {connections, weight: OUTER_CORNER_WEIGHT, texture: "Wall TL", flipx: false, flipy: false, rot90: false});
        let connections = [c::Sky, c::Sky, c::WallRight, c::WallTop];
        possible_tiles.push(TileChoice {connections, weight: OUTER_CORNER_WEIGHT, texture: "Wall TR", flipx: false, flipy: false, rot90: false});
        let connections = [c::WallLeft, c::WallInner, c::WallInner, c::WallTop];
        possible_tiles.push(TileChoice {connections, weight: INNER_CORNER_WEIGHT, texture: "Wall LC", flipx: false, flipy: false, rot90: false});
        let connections = [c::WallRight, c::WallTop, c::WallInner, c::WallInner];
        possible_tiles.push(TileChoice {connections, weight: INNER_CORNER_WEIGHT, texture: "Wall RC", flipx: false, flipy: false, rot90: false});

        //Sky 
        let connections = [c::Sky, c::Sky, c::Sky, c::Sky];
        possible_tiles.push(TileChoice {connections, weight: 1, texture: "Sky", flipx: false, flipy: false, rot90: false});


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