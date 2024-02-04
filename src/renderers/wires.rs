use crate::*;

const MARGIN1: f32 = 0.33333;
const MARGIN2: f32 = 1.0-MARGIN1;

pub fn draw_tilegrid (grid: &TileGrid) {
    for i in 0..grid.width {
        for j in 0..grid.height {
            let tile = &grid.tilegrid[i as usize][j as usize];
            let tileopt = tile.possible_tiles.choose(&mut ::rand::thread_rng()).unwrap();
            for k in 0..4 {
                let connection = tileopt.connections[k];
                let color = match connection {
                    Connection::Red => RED,
                    Connection::Green => GREEN,
                    Connection::Blue => BLUE,
                    Connection::Yellow => GOLD,
                    Connection::White => WHITE,
                    Connection::Black => {continue;},
                };
                let tx = (i as f32) * grid.tilewidth + grid.marginx;
                let ty = (j as f32) * grid.tileheight + grid.marginy;
                draw_rectangle(tx+(grid.tilewidth/3.0), ty+(grid.tileheight/3.0), grid.tilewidth/3.0, grid.tileheight/3.0, color);
            }
            for k in 0..4 {
                let connection = tileopt.connections[k];

                if connection == Connection::Black {
                    continue;
                }
                let color = match connection {
                    Connection::Red => RED,
                    Connection::Green => GREEN,
                    Connection::Blue => BLUE,
                    Connection::Yellow => GOLD,
                    Connection::White => WHITE,
                    Connection::Black => BLACK,
                };

                let tx = (i as f32) * grid.tilewidth + grid.marginx;
                let ty = (j as f32) * grid.tileheight + grid.marginy;
                let (x, y) = match k {
                    0 => (tx+(grid.tilewidth/3.0), ty),
                    1 => (tx+(grid.tilewidth/3.0), ty+(grid.tileheight/1.5)),
                    2 => (tx, ty+(grid.tileheight/3.0)),
                    3 => (tx+(grid.tilewidth/1.5), ty+(grid.tileheight/3.0)),
                    _ => (tx, ty),
                };
                draw_rectangle(x, y, grid.tilewidth/3.0, grid.tileheight/3.0, color);
                
                // tiny center triangles
                let tx = (i as f32) * grid.tilewidth + grid.marginx;
                let ty: f32 = (j as f32) * grid.tileheight + grid.marginy;
                let tl = Vec2::new(tx+grid.tilewidth*MARGIN1, ty+grid.tileheight*MARGIN1);
                let tr = Vec2::new(tx+grid.tilewidth*MARGIN2, ty+grid.tileheight*MARGIN1);
                let bl = Vec2::new(tx+grid.tilewidth*MARGIN1, ty+grid.tileheight*MARGIN2);
                let br = Vec2::new(tx+grid.tilewidth*MARGIN2, ty+grid.tileheight*MARGIN2);
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
