/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "mixer/stream.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

use Audio;

pub struct Stream<'a> {
	audio: &'a Audio,
	curs: usize,
}

impl<'a> Stream<'a> {
	pub fn create(audio: &'a Audio) -> Stream<'a> {
		Stream {
			audio: audio,
			curs: 0,
		}
	}

	pub fn update(&mut self) -> (i16, bool) {
		let curs = self.curs;

		self.curs += 1;

		(self.audio[curs], self.curs == self.audio.len())
	}
}

impl<'a> PartialEq<Audio> for Stream<'a> {
	fn eq(&self, other: &Audio) -> bool {
		self.audio as *const _ == other as *const _
	}
}
