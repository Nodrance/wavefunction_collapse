use crate::*;
use std::collections::HashMap;
use std::hash::Hash;

/// Load textures from a folder (not ending in /), using the associated paths as filenames.
/// Usage:
/// ```no_run
/// let textures = load_textures_paths(&["assets/wall.png","assets/floor.png"], &["wall","floor"]).await;
/// draw_texture(textures.get("wall").unwrap(), 0.0, 0.0, WHITE);
/// ```
pub async fn load_textures_paths<T: Hash + Eq + Clone>(paths: &[&str], keys: &[T]) -> HashMap<T, Texture2D> {

    let mut hashmap = HashMap::new();
    let missing_texture = load_texture("missing.png").await.unwrap();

    for i in 0..paths.len() {
        if keys.len() <= i {
            break;
        }
        let dir = paths.get(i);
        if let Some(dir) = dir {
            let texture = load_texture(dir).await;
            if let Ok(texture) = texture {
                texture.set_filter(FilterMode::Nearest);
                hashmap.insert(keys[i].clone(), texture);
            }
            else {
                println!("Failed to load texture from path: {}", dir);
                hashmap.insert(keys[i].clone(), missing_texture.clone());
            }
        }
        else {
            println!("Not enough paths provided to load_textures_paths");
            hashmap.insert(keys[i].clone(), missing_texture.clone());
        }
    }
    return hashmap;
}

/// Load textures from a folder (not ending in /), using the stringed keys as filenames.
/// Usage:
/// ```no_run
/// let textures = load_textures("assets/island", &["grass", "water", "beach1", "beach2"]).await;
/// draw_texture(textures.get("grass").unwrap(), 0.0, 0.0, WHITE);
/// ```
pub async fn load_textures_stringable<T: Hash + Eq + Clone + ToString>(folder: &str, keys: &[T]) -> HashMap<T, Texture2D> {

    let mut hashmap = HashMap::new();
    let missing_texture = load_texture("missing.png").await.unwrap();

    for key in keys {
        let texture = load_texture(
            format!("{}/{}.png", folder, key.to_string()).as_str()
        ).await;
        if let Ok(texture) = texture {
            texture.set_filter(FilterMode::Nearest);
            hashmap.insert(key.clone(), texture);
        }
        else {
            hashmap.insert(key.clone(), missing_texture.clone());
        }
    }
    return hashmap;
}

pub fn draw_tilegrid (grid: &TileGrid, textures: &HashMap<&str, Texture2D>, tile_size: Vec2, texture_limits: Vec2, offset: i32, render_every: i32) {
    for i in 0..grid.width {
        for j in 0..grid.height {
            // render 1 in render_every tiles
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

pub fn draw_tile_opt (x: f32, y: f32, tile_size: Vec2, tileopt: &TileChoice, textures: &HashMap<&str, Texture2D>) {
    let texture = textures.get(tileopt.texture).unwrap();
    let dest_size = if tileopt.rot90 {Vec2::new(tile_size.y, tile_size.x)} else {tile_size};
    // textures are rotated at their center after scaling, which won't be the same as the tile's center
    let x = x + if tileopt.rot90 {(tile_size.x-tile_size.y)/2.0} else {0.0};
    let y = y + if tileopt.rot90 {(tile_size.y-tile_size.x)/2.0} else {0.0};
    let params = DrawTextureParams {
        dest_size: Some(dest_size),
        rotation: if tileopt.rot90 {std::f32::consts::FRAC_PI_2} else {0.0},
        flip_x: tileopt.flipx,
        flip_y: tileopt.flipy,
        ..Default::default()};
    draw_texture_ex(texture, x, y, WHITE, params);
}
