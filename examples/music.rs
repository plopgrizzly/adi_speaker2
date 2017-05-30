/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "examples/music.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

extern crate adi_speaker;

use adi_speaker::{ Speaker, Audio };

fn main() {
	println!("Open Speaker");

	let mut speaker = Speaker::create();

	println!("Load Audio");

	let music = Audio::create(include_bytes!("res/197_DrownNoMore.raw"));

	println!("Start Music");

	speaker.play(&music, 60.0 * 0.975, 0.0);

	println!("OK");

	while speaker.is_playing(&music) {
		speaker.update();
	}

	println!("Done");
}
