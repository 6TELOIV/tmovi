use std::{env, fs::File, io::{BufWriter, Write}};

use quote::quote;

/// Loads maps and tilesets from Tiled, and generates rust code with constants
/// for use in the game.
fn main() {
    // Provided by cargo; where the generated code should go
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable must be specified");

    // Don't rebuild if the tilemaps haven't changed (will check for any
    // modifications in the maps folder)
    println!("cargo:rerun-if-changed=maps");

    // Load the maps
    let mut loader = tiled::Loader::new();
    let metrometropolis = loader.load_tmx_map("maps/metrometropolis.tmx").unwrap();

    let width = metrometropolis.width;
    let height = metrometropolis.height;

    let metrometropolis_tile_layer = metrometropolis.get_layer(0).unwrap().as_tile_layer().unwrap();
    let metrometropolis_tiles = extract_tiles(&metrometropolis_tile_layer);

    let output = quote! {
        pub static METROMETROPOLIS_MAP: &[u16] = &[#(#metrometropolis_tiles),*];
        pub const WIDTH: u16 = #width as u16;
        pub const HEIGHT: u16 = #height as u16;
    };

    let output_file = File::create(format!("{out_dir}/tilemap.rs"))
        .expect("failed to open tilemap.rs file for writing");
    let mut writer = BufWriter::new(output_file);

    write!(&mut writer, "{output}").unwrap();
}

/// Takes a Tiled tile layer and returns an array of tile ids
fn extract_tiles<'a>(tile_layer: &tiled::TileLayer) -> impl Iterator<Item = u16> + 'a {
    let mut result = vec![];
    for y in 0..tile_layer.height().unwrap() {
        for x in 0..tile_layer.width().unwrap() {
            result.push(match tile_layer.get_tile(x as i32, y as i32) {
                Some(tile) => tile.id() as u16,
                None => 0
            });
        }
    }
    result.into_iter()
}