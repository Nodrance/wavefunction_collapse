use crate::*;

pub fn draw_tilegrid (grid: &TileGrid, textures: &HashMap<&str, Texture2D>, tile_size: Vec2, texture_limits: Vec2, offset: i32, render_every: i32) {
    for i in 0..grid.height+1 {
        draw_line(0.0, (i as f32) * tile_size.y,
                  grid.width as f32*tile_size.x, (i as f32) * tile_size.y,
                  1.0, WHITE);
    }
    for i in 0..grid.width+1 {
        draw_line((i as f32) * tile_size.x, 0.0,
                  (i as f32) * tile_size.x, grid.height as f32*tile_size.y, 
                  1.0, WHITE);
    }

}
