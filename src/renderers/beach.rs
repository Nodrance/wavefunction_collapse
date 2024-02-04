use macroquad::texture;

use crate::*;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

pub async fn load_textures_unordered<T: Hash + Eq + Clone>(path: &str, keys: &[T]) -> HashMap<T, Texture2D> {

    let mut hashmap = HashMap::new();
    let missing_texture = load_texture("missing.png").await.unwrap();
    let paths_result = fs::read_dir(path);

    if let Ok(paths) = paths_result {
        for (i, maybe_dir) in paths.enumerate() {
            if keys.len() <= i {
                break;
            }
            if let Ok(dir) = maybe_dir {
                let texture = load_texture(dir.path().to_str().unwrap()).await;
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
    }
    else {
        for key in keys {
            hashmap.insert(key.clone(), missing_texture.clone());
        }
    }
    return hashmap;
}

pub async fn load_textures_string<T: Hash + Eq + Clone + ToString>(path: &str, keys: &[T]) -> HashMap<T, Texture2D> {

    let mut hashmap = HashMap::new();
    let missing_texture = load_texture("missing.png").await.unwrap();

    for key in keys {
        let texture = load_texture(
            format!("{}/{}.png", path, key.to_string()).as_str()
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

pub fn draw_tilegrid (grid: &TileGrid, textures: &HashMap<TileChoice, Texture2D>) {
    for i in 0..grid.width {
        for j in 0..grid.height {
            let tile = &grid.tilegrid[i as usize][j as usize];
            let tileopt = tile.possible_tiles.choose(&mut ::rand::thread_rng()).unwrap();
            let tx = (i as f32) * grid.tilewidth + grid.marginx;
            let ty = (j as f32) * grid.tileheight + grid.marginy;
            let texture = textures.get(tileopt).unwrap();
            draw_texture_ex(texture, tx, ty, RED, DrawTextureParams {
                dest_size: Some(Vec2::new(grid.tilewidth, grid.tileheight)),
                ..Default::default()
            });
        }
    }
}
