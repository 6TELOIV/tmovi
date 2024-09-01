//! Grotto Beasts - The Mystery of Volca Isle
//!
//! This is a Jerma985/Grotto Beasts fan game for the Gameboy Advance written with the
//! `agb` library.

// Games made using `agb` are no_std which means you don't have access to the standard
// rust library.
#![no_std]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{display::{object::{self, Graphics, Sprite}, tile_data::TileData, tiled::{RegularBackgroundSize, TileSetting, TiledMap}, Priority}, include_aseprite, input, interrupt::VBlank };

mod tilemap {
    include!(concat!(env!("OUT_DIR"), "/tilemap.rs"));
}

fn get_meta_tile<'a>(meta_index: u16, tile_data: &'a TileData) -> &'a [TileSetting] {
    let real_index = (meta_index * 8) as usize;
    tile_data.tile_settings.get(real_index..(real_index + 8)).unwrap()
}

// Include Graphics
agb::include_background_gfx!(backgrounds, "000000", metrometropolis => deduplicate "gfx/world/backgrounds/metrometropolis.aseprite");
static GRAPHICS: &Graphics = include_aseprite!("gfx/world/sprites/player.aseprite");
static PLAYER: &Sprite = GRAPHICS.tags().get("Player").sprite(0);

pub fn run(mut gba: agb::Gba) -> ! {
    // System
    let vblank = VBlank::get();
    let object: object::OamManaged<'_> = gba.display.object.get_managed();
    let mut input = input::ButtonController::new();
    let (gfx, mut vram) = gba.display.video.tiled0();

    // Make sprite
    let mut player = object.object_sprite(PLAYER);
    player.set_x(50).set_y(50).show();

    // Set background
    vram.set_background_palettes(backgrounds::PALETTES);
    let mut map_0 = gfx.background(Priority::P0, RegularBackgroundSize::Background64x64, backgrounds::metrometropolis.tiles.format());
    let mut map_1 = gfx.background(Priority::P1, RegularBackgroundSize::Background64x64, backgrounds::metrometropolis.tiles.format());
    
    for y in 0..tilemap::HEIGHT.clamp(0, 64) {
        let is_even = y % 2 == 0;
        let map_to_draw = if is_even { &mut map_0 } else { &mut map_1 };
        for x in 0..tilemap::WIDTH.clamp(0, 64) {
            let tilemap_index = (x + (y * tilemap::WIDTH)) as usize;
            let meta_index = tilemap::METROMETROPOLIS_MAP[tilemap_index];
            let tile_settings = get_meta_tile(meta_index, &backgrounds::metrometropolis);
            
            let real_x = (x * 4) + if is_even {2} else {0};
            for (index, tile_setting) in tile_settings[0..4].iter().enumerate() {
                map_to_draw.set_tile(&mut vram, (real_x + index as u16, y), &backgrounds::metrometropolis.tiles, *tile_setting);
            }
            for (index, tile_setting) in tile_settings[4..8].iter().enumerate() {
                map_to_draw.set_tile(&mut vram, (real_x + index as u16, y + 1), &backgrounds::metrometropolis.tiles, *tile_setting);
            }
        }
    }

    map_0.commit(&mut vram);
    map_0.set_visible(true);
    map_1.commit(&mut vram);
    map_1.set_visible(true);

    // Main Loop
    loop {
        // Frame updates
        vblank.wait_for_vblank();
        object.commit();
        input.update();
    }
}