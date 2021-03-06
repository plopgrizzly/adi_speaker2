/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "examples/music.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

extern crate adi_speaker;

use adi_speaker::{ Speaker, Audio };

fn main() {
	println!("Load Audio");

	let trombone = Audio::create(include_bytes!("res/197_DrownNoMore.raw"));
//	let music = Audio::create(include_bytes!("res/197_DrownNoMore.raw"));
//	let cowbell = Audio::create(include_bytes!("res/cowbell.raw"));
//	let vibraslap = Audio::create(include_bytes!("res/vibraslap.raw"));

	println!("Open Speaker");

	let mut speaker = Speaker::create();

	println!("Start Music");

/*	speaker.play(&music, 60.0 * 0.975, 0.0);

	println!("OK");

	while speaker.is_playing(&music) {
		speaker.update();
	}

	println!("Done");*/

/*	speaker.play(&music, 60.0 * 0.975, 0.0);

	while speaker.is_playing(&music) {
		speaker.update();
	}*/

	println!("vibraslappy");

	speaker.play(&trombone);
//		.volume(&mut speaker, (0.0, 5.0), 0.0)
//		.fade_in(&mut speaker, (0.0, 10.0))
//		.fade_out(&mut speaker, (0.0, 10.0));
//		.echo(&mut speaker, (0.0, 10.0), (0.5, 1.0));
	while speaker.is_playing(&trombone) { speaker.update(); }
/*
	speaker.play(&trombone, 0.0, 0.0);
	while speaker.is_playing(&trombone) { speaker.update(); }

	speaker.play(&vibraslap, 0.0, 0.0);
	while speaker.is_playing(&vibraslap) { speaker.update(); }*/
}
