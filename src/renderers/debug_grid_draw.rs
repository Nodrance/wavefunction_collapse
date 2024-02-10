use crate::*;

const MARGIN1A: f32 = 0.25; ///Distance from the edge of the tile to the edge of the tile identifier square
const MARGIN1B: f32 = 1.0-(2.0*MARGIN1A); ///Size of the tile identifier square
const MARGIN2A: f32 = 0.00; ///Distance from the edge of the tile to the edge of the connection triangles
const MARGIN2B: f32 = 1.0-MARGIN2A; ///Opposite corner of the triangle

use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

pub fn draw_tilegrid (grid: &TileGrid, textures: &HashMap<&str, Texture2D>, tile_size: Vec2, texture_limits: Vec2, offset: i32, render_every: i32) {
    for i in 0..grid.width {
        for j in 0..grid.height {
            if (i*101 + j*5)%render_every != offset%render_every {
                continue;
            }
            let tx = (i as f32) * tile_size.x;
            let ty = (j as f32) * tile_size.y;
            // don't render tiles that are offscreen
            if tx < -tile_size.x || ty < -tile_size.y || tx > texture_limits.x || ty > texture_limits.y {
                continue;
            }

            let tile = &grid.tilegrid[i as usize][j as usize];
            // alternate choice methods
            // let tileopt = &tile.possible_tiles[rand as usize % tile.possible_tiles.len()];
            // let tileopt = &tile.possible_tiles[0];
            let tileopt = if tile.possible_tiles.len() == 1 {&tile.possible_tiles[0]}
            else {tile.possible_tiles.choose(&mut ::rand::thread_rng()).unwrap()};
            draw_tile_opt(tx, ty, tile_size, tileopt, textures);
        }
    }
    draw_rectangle(grid.width as f32 * tile_size.x, 0.0, grid.width as f32 * tile_size.x, grid.height as f32 * tile_size.y * 2.0, BLACK);
    draw_rectangle(0.0, grid.height as f32 * tile_size.y, grid.width as f32 * tile_size.x, grid.height as f32 * tile_size.y, BLACK);
}

fn draw_tile_opt (x: f32, y: f32, tile_size: Vec2, tileopt: &TileChoice, textures: &HashMap<&str, Texture2D>) {
    for k in 0..4 {
        let connection = tileopt.connections[k];
        let mut hasher = DefaultHasher::new();
        connection.hash(&mut hasher);
        let color = match hasher.finish() % 16 {
            0 => RED,
            1 => ORANGE,
            2 => YELLOW,
            3 => GREEN,
            4 => DARKGREEN,
            5 => SKYBLUE,
            6 => DARKBLUE,
            7 => PINK,
            8 => VIOLET,
            9 => DARKPURPLE,
            10 => BROWN,
            11 => DARKBROWN,
            12 => WHITE,
            13 => GRAY,
            14 => DARKGRAY,
            _ => BLACK,
        };
        let tl = Vec2::new(x+tile_size.x*MARGIN2A, y+tile_size.y*MARGIN2A);
        let tr = Vec2::new(x+tile_size.x*MARGIN2B, y+tile_size.y*MARGIN2A);
        let bl = Vec2::new(x+tile_size.x*MARGIN2A, y+tile_size.y*MARGIN2B);
        let br = Vec2::new(x+tile_size.x*MARGIN2B, y+tile_size.y*MARGIN2B);
        let center = Vec2::new(x+(tile_size.x/2.0), y+(tile_size.y/2.0));
        let (v1, v2, v3) = match k {
            0 => (tl, tr, center),
            1 => (tr, br, center),
            2 => (br, bl, center),
            _ => (bl, tl, center),
        };
        draw_triangle(v1, v2, v3, color);
    }

    {
        let mut hasher = DefaultHasher::new();
        tileopt.hash(&mut hasher);
        let color = match hasher.finish() % 16 {
            0 => RED,
            1 => ORANGE,
            2 => YELLOW,
            3 => GREEN,
            4 => DARKGREEN,
            5 => SKYBLUE,
            6 => DARKBLUE,
            7 => PINK,
            8 => VIOLET,
            9 => DARKPURPLE,
            10 => BROWN,
            11 => DARKBROWN,
            12 => WHITE,
            13 => GRAY,
            14 => DARKGRAY,
            _ => BLACK,
        };
        draw_rectangle(x+(tile_size.x*MARGIN1A), y+(tile_size.y*MARGIN1A), tile_size.x*MARGIN1B, tile_size.y*MARGIN1B, color);
    }
}
