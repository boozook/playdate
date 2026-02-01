#![no_std]
#![no_main]
#![allow(unused_must_use)]

#[macro_use]
extern crate sys;
extern crate playdate_graphics as gfx;


use display::Display;
use fs::path::Path;
use gfx::bitmap::table::BitmapTable;
use gfx::bitmap::tilemap::TileMap;
use system::System;
use sys::ffi::{Playdate, SystemEvent};
use sys::ffi::{LCD_COLUMNS, LCD_ROWS};
use sys::ctrl::EventLoopCtrl;

use gfx::Graphics;


const CENTER_X: i32 = LCD_COLUMNS as i32 / 2;
const CENTER_Y: i32 = LCD_ROWS as i32 / 2;
const TILEMAP_FILENAME: &Path = c"tiles";


#[unsafe(no_mangle)]
fn event_handler(api: &'static Playdate, e: SystemEvent, _: u32) -> EventLoopCtrl {
	let SystemEvent::Init = dbg!(e) else {
		return EventLoopCtrl::Continue;
	};

	Display::new(api.display).set_fps(0.);
	System::new(api.system).update().unset();


	// api endpoints
	let gfx = Graphics::new(api.graphics);
	let tm = gfx.tilemap();


	// load spritesheet / bitmap-table, use it in multiple tilemaps
	let mut tiles = BitmapTable::load(&gfx, TILEMAP_FILENAME).unwrap();

	// render bitmap-table as tilemap as-is
	let mut tmap = as_is(gfx, &mut tiles);
	tmap.set_image_table(&tm, &mut tiles);
	let (_, h) = tmap.pixel_size(&tm);
	tmap.draw(&tm, 0., (CENTER_Y - h as i32 / 2) as _);

	// build small tilemap and render
	let mut tmap = neat(gfx, &mut tiles);
	tmap.set_image_table(&tm, &mut tiles);
	let (_, h) = tmap.pixel_size(&tm);
	tmap.draw(&tm, CENTER_X as _, (CENTER_Y - h as i32 / 2) as _);


	EventLoopCtrl::Continue
}


fn as_is(gfx: Graphics, tiles: &mut BitmapTable) -> TileMap {
	let (num, w) = tiles.info(&gfx);
	let h = num / w;

	let tm = gfx.tilemap();

	let mut tmap = TileMap::new(&tm).unwrap();
	tmap.set_image_table(&tm, tiles);

	tmap.set_size(&tm, w, h);

	(0..h).flat_map(|y| (0..w).map(move |x| (x, y)))
	      .enumerate()
	      .for_each(|(i, (x, y))| tmap.set_tile_at(&tm, x, y, i as _));
	tmap
}


fn neat(gfx: Graphics, tiles: &mut BitmapTable) -> TileMap {
	let tm = gfx.tilemap();

	let mut tmap = TileMap::new(&tm).unwrap();
	tmap.set_image_table(&tm, tiles);

	let (w, h) = (5, 3);
	tmap.set_size(&tm, w, h);

	let rows = [[8, 9, 9, 9, 10], [16, 17, 25, 17, 18], [24, 26, 15, 24, 26]];

	(0..h).flat_map(|y| (0..w).map(move |x| (x, y)))
	      .for_each(|(x, y)| tmap.set_tile_at(&tm, x, y, rows[y as usize][x as usize]));
	tmap
}
