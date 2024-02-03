use crate::*;

pub fn draw_tilegrid (grid: &TileGrid) {
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
