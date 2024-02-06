use crate::*;

const MARGIN1A: f32 = 0.25; ///Distance from the edge of the tile to the edge of the tile identifier square
const MARGIN1B: f32 = 1.0-(2.0*MARGIN1A); ///Size of the tile identifier square
const MARGIN2A: f32 = 0.0; ///Distance from the edge of the tile to the edge of the connection triangles
const MARGIN2B: f32 = 1.0-MARGIN2A; ///Opposite corner of the triangle

use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

pub fn draw_tilegrid (grid: &TileGrid) {
    // for i in 0..grid.width {
    //     for j in 0..grid.height {
    //         let tile = &grid.tilegrid[i as usize][j as usize];
    //         let tileopt = tile.possible_tiles.choose(&mut ::rand::thread_rng()).unwrap();
    //         for k in 0..4 {
    //             let connection = tileopt.connections[k];

    //             let mut hasher = DefaultHasher::new();
    //             connection.hash(&mut hasher);
    //             let color = match hasher.finish() % 16 {
    //                 0 => RED,
    //                 1 => ORANGE,
    //                 2 => YELLOW,
    //                 3 => GREEN,
    //                 4 => DARKGREEN,
    //                 5 => SKYBLUE,
    //                 6 => DARKBLUE,
    //                 7 => PINK,
    //                 8 => VIOLET,
    //                 9 => DARKPURPLE,
    //                 10 => BROWN,
    //                 11 => DARKBROWN,
    //                 12 => WHITE,
    //                 13 => GRAY,
    //                 14 => DARKGRAY,
    //                 _ => BLACK,
    //             };

    //             let tx = (i as f32) * grid.tilewidth + grid.marginx;
    //             let ty: f32 = (j as f32) * grid.tileheight + grid.marginy;
    //             let tl = Vec2::new(tx+grid.tilewidth*MARGIN2A, ty+grid.tileheight*MARGIN2A);
    //             let tr = Vec2::new(tx+grid.tilewidth*MARGIN2B, ty+grid.tileheight*MARGIN2A);
    //             let bl = Vec2::new(tx+grid.tilewidth*MARGIN2A, ty+grid.tileheight*MARGIN2B);
    //             let br = Vec2::new(tx+grid.tilewidth*MARGIN2B, ty+grid.tileheight*MARGIN2B);
    //             let center = Vec2::new(tx+(grid.tilewidth/2.0), ty+(grid.tileheight/2.0));
    //             let (v1, v2, v3) = match k {
    //                 0 => (tr, tl, center),
    //                 1 => (bl, br, center),
    //                 2 => (tl, bl, center),
    //                 _ => (br, tr, center),
    //             };
    //             draw_triangle(v1, v2, v3, color);
    //         }

    //         {
    //             let mut hasher = DefaultHasher::new();
    //             tileopt.hash(&mut hasher);
    //             let color = match hasher.finish() % 16 {
    //                 0 => RED,
    //                 1 => ORANGE,
    //                 2 => YELLOW,
    //                 3 => GREEN,
    //                 4 => DARKGREEN,
    //                 5 => SKYBLUE,
    //                 6 => DARKBLUE,
    //                 7 => PINK,
    //                 8 => VIOLET,
    //                 9 => DARKPURPLE,
    //                 10 => BROWN,
    //                 11 => DARKBROWN,
    //                 12 => WHITE,
    //                 13 => GRAY,
    //                 14 => DARKGRAY,
    //                 _ => BLACK,
    //             };
    //             let tx = (i as f32) * grid.tilewidth + grid.marginx;
    //             let ty = (j as f32) * grid.tileheight + grid.marginy;
    //             draw_rectangle(tx+(grid.tilewidth*MARGIN1A), ty+(grid.tileheight*MARGIN1A), grid.tilewidth*MARGIN1B, grid.tileheight*MARGIN1B, color);
    //         }
            
    //     }
    // }
}
