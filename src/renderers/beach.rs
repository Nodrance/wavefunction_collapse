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
                hashmap.insert(keys[i].clone(), texture);
            }
            else {
                hashmap.insert(keys[i].clone(), missing_texture.clone());
            }
        }
        else {
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
            hashmap.insert(key.clone(), texture);
        }
        else {
            hashmap.insert(key.clone(), missing_texture.clone());
        }
    }
    return hashmap;
}

pub fn draw_tilegrid (grid: &TileGrid, textures: &HashMap<&str, Texture2D>) {
    for i in 0..grid.width {
        for j in 0..grid.height {
            let tile = &grid.tilegrid[i as usize][j as usize];
            let tileopt = tile.possible_tiles.choose(&mut ::rand::thread_rng()).unwrap();
            let tx = (i as f32) * grid.tilewidth + grid.marginx;
            let ty = (j as f32) * grid.tileheight + grid.marginy;
            let texture = textures.get(tileopt.texture).unwrap();
            let params = DrawTextureParams {
                dest_size: Some(Vec2::new(grid.tilewidth, grid.tileheight)),
                rotation: if tileopt.rot90 {std::f32::consts::FRAC_PI_2} else {0.0},
                flip_x: tileopt.flipx,
                flip_y: tileopt.flipy,
                ..Default::default()};
            draw_texture_ex(texture, tx, ty, WHITE, params);
        }
    }
}
