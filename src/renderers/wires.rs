use crate::*;

pub fn draw_tilegrid (grid: &TileGrid) {
    for i in 0..grid.width {
        for j in 0..grid.height {
            let tile = &grid.tilegrid[i as usize][j as usize];
            let tileopt = tile.possible_tiles.choose(&mut ::rand::thread_rng()).unwrap();
            for k in 0..4 {
                let connection = tileopt.connections[k];

                if connection == Connection::Black {
                    continue;
                }
                let color = match connection {
                    Connection::Red => RED,
                    Connection::Green => GREEN,
                    Connection::Blue => BLUE,
                    Connection::Yellow => YELLOW,
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
                draw_rectangle(tx+(grid.tilewidth/3.0), ty+(grid.tileheight/3.0), grid.tilewidth/3.0, grid.tileheight/3.0, color)
            }
        }
    }
}
