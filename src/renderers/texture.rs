use crate::*;

// load textures somehow

pub fn draw_tilegrid (grid: &TileGrid, rendermode: Rendermode) {
    for i in 0..grid.width {
        for j in 0..grid.height {
            let tile = &grid.tilegrid[i as usize][j as usize];
            let tileopt = tile.possible_tiles.choose(&mut ::rand::thread_rng()).unwrap();
            let tx = (i as f32) * grid.tilewidth + grid.marginx;
            let ty = (j as f32) * grid.tileheight + grid.marginy;
            let texture = None; // somehow get the texture
            draw_texture_ex(texture, tx, ty, color, DrawTextureParams {
                dest_size: Some(Vec2::new(grid.tilewidth, grid.tileheight)),
                ..Default::default()
            });
        }
    }
}
