use macroquad::prelude::*;
use ::rand::seq::SliceRandom;
use ::rand::distributions::WeightedIndex;
use ::rand::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Connection {
    Black,
    RedWire,
    BlueWire,
    GreenWire,
    YellowWire,
    WhiteWire,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Tile {
    connections: [Connection; 4],
}
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct UndecidedTile {
    possible_tiles: Vec<Tile>,
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

impl UndecidedTile {
    fn new() -> Self {
        let mut possible_tiles = Vec::<Tile>::new();
        for connection in [Connection::RedWire, Connection::BlueWire, Connection::GreenWire, Connection::YellowWire, Connection::WhiteWire].iter() {
            for i in 1..16 {
                let mut tile = Tile { connections: [Connection::Black; 4] };
                let mut conns = 0;
                if i & 1 == 1 {
                    tile.connections[0] = connection.clone();
                    conns += 1;
                }
                if i & 2 == 2 {
                    tile.connections[1] = connection.clone();
                    conns += 1;
                }
                if i & 4 == 4 {
                    tile.connections[2] = connection.clone();
                    conns += 1;
                }
                if i & 8 == 8 {
                    tile.connections[3] = connection.clone();
                    conns += 1;
                } 
                if conns > 0 {
                    possible_tiles.push(tile);
                }
            }
        }
        Self {
            possible_tiles: possible_tiles,
        }
    }
}


fn draw_grid (grid: &TileGrid) {
    for i in 0..grid.height+1 {
        draw_line(grid.marginx, (i as f32) * grid.tileheight + grid.marginy,
                  (grid.width as f32) * grid.tilewidth + grid.marginx, (i as f32) * grid.tileheight + grid.marginy,
                  1.0, WHITE);
    }
    for i in 0..grid.width+1 {
        draw_line((i as f32) * grid.tilewidth + grid.marginx, grid.marginy,
                  (i as f32) * grid.tilewidth + grid.marginx, (grid.height as f32) * grid.tileheight + grid.marginy, 
                  1.0, WHITE);
    }
    for i in 0..grid.width {
        for j in 0..grid.height {
            let tile = &grid.tilegrid[i as usize][j as usize];
            for k in 0..4 {
                let connection = tile.possible_tiles.choose(&mut ::rand::thread_rng()).unwrap().connections[k];
                let color = match connection {
                    Connection::Black => continue,
                    Connection::RedWire => RED,
                    Connection::BlueWire => BLUE,
                    Connection::GreenWire => GREEN,
                    Connection::YellowWire => YELLOW,
                    Connection::WhiteWire => WHITE,
                };
                let tx = (i as f32) * grid.tilewidth + grid.marginx;
                let ty = (j as f32) * grid.tileheight + grid.marginy;
                if false { // ribbon style
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
                if true { // triangle style
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
}

fn propegate_changes (grid: &mut TileGrid, x: i32, y: i32) {
    // collapse the tile at the given index, and propegate the collapse to the neighbors
    let mut todo_indices = vec![(x, y), (x-1, y), (x+1, y), (x, y-1), (x, y+1)];
    let mut index = 0;
    while index < todo_indices.len() {
        let (x, y) = todo_indices[index];
        index += 1;
        if x < 0 || x >= grid.width || y < 0 || y >= grid.height {
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
            if neighbor_direction_indices.0 < 0 || neighbor_direction_indices.0 >= grid.width || neighbor_direction_indices.1 < 0 || neighbor_direction_indices.1 >= grid.height {
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
                let neighbor_tile_option = neighbor_tile.possible_tiles[neighbor_tile_option_index];
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
    // create a grid of new undecided tiles
    let mut grid = TileGrid{
        tilegrid: Vec::<Vec<UndecidedTile>>::new(),
        width: 10,
        height: 10,
        tilewidth: 25.0,
        tileheight: 25.0,
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
        let mut candidate_indices = Vec::<(i32, i32)>::new();
        let mut least_seen = 100000;
        let mut weights = Vec::<i32>::new();
        let mut total_seen = 0;
        const LARGE_WEIGHT: i32 = 1;
        for i in 0..grid.width {
            for j in 0..grid.height {
                let tile = &grid.tilegrid[i as usize][j as usize];
                if tile.possible_tiles.len() == 1 {
                    continue;
                }
                total_seen += 1;
                if tile.possible_tiles.len() < least_seen {
                    least_seen = tile.possible_tiles.len();
                    weights = vec![1;total_seen-1];
                    weights.push(LARGE_WEIGHT);
                }
                else if tile.possible_tiles.len() == least_seen {
                    weights.push(LARGE_WEIGHT);
                }
                else {
                    weights.push(1);
                }
                candidate_indices.push((i, j));
            }
        }

        if candidate_indices.len() == 0 {
            // We're done, skip the heavy processing
        }
        else {
            let dist = WeightedIndex::new(&weights).unwrap();
            let (x_index, y_index) = candidate_indices[dist.sample(&mut ::rand::thread_rng())];
            let mut tile = grid.tilegrid[x_index as usize][y_index as usize].clone();
            tile = UndecidedTile{possible_tiles: vec![*tile.possible_tiles.choose(&mut ::rand::thread_rng()).unwrap()]};
            grid.tilegrid[x_index as usize][y_index as usize] = tile;
            propegate_changes(&mut grid, x_index, y_index);
        }
        next_frame().await
    }
}