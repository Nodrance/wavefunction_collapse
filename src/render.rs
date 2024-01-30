use crate::*;

pub fn draw_tilegrid (grid: &TileGrid, rendermode: Rendermode) {
    for i in 0..grid.width {
        for j in 0..grid.height {
            let tile = &grid.tilegrid[i as usize][j as usize];
            let tileopt = tile.possible_tiles.choose(&mut ::rand::thread_rng()).unwrap();
            for k in 0..4 {
                let connection = tileopt.connections[k];

                if connection == Connection::Black {
                    continue;
                }
                let color = tileopt.color;

                let tx = (i as f32) * grid.tilewidth + grid.marginx;
                let ty = (j as f32) * grid.tileheight + grid.marginy;
                if rendermode == Rendermode::Texture && tileopt.texture.is_some() {
                    let texture = tileopt.texture.as_ref().unwrap();
                    draw_texture_ex(texture, tx, ty, color, DrawTextureParams {
                        dest_size: Some(Vec2::new(grid.tilewidth, grid.tileheight)),
                        ..Default::default()
                    });
                }
                else if rendermode == Rendermode::Ribbon {
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
                else if rendermode == Rendermode::Triangle {
                    const MARGIN1: f32 = 1.0;
                    const MARGIN2: f32 = 1.0-MARGIN1;
                    let tl = Vec2::new(tx+grid.tilewidth*MARGIN2, ty+grid.tileheight*MARGIN2);
                    let tr = Vec2::new(tx+grid.tilewidth*MARGIN1, ty+grid.tileheight*MARGIN2);
                    let bl = Vec2::new(tx+grid.tilewidth*MARGIN2, ty+grid.tileheight*MARGIN1);
                    let br = Vec2::new(tx+grid.tilewidth*MARGIN1, ty+grid.tileheight*MARGIN1);
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
}
