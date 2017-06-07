/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "mixer/stream.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

use Audio;

pub struct Transform {
	run: fn(&mut f32, &Audio, usize, f32) -> (),
	range: (usize, f32),
}

pub struct Stream<'a> {
	audio: &'a Audio,
	curs: usize,
	transforms: Vec<Transform>,
}

impl<'a> Stream<'a> {
	pub fn create(audio: &'a Audio) -> Stream<'a> {
		Stream {
			audio: audio,
			curs: 0,
			transforms: Vec::new(),
		}
	}

	pub fn update(&mut self) -> (f32, bool) {
		let mut sample = self.audio.sample(self.curs);

		for transform in &self.transforms {
			let animate = (self.curs - transform.range.0) as f32 /
				transform.range.1;

			// Not applied to this range.
			if animate < 0.0 || animate > 1.0 { continue; }

			(transform.run)(&mut sample, &self.audio, self.curs,
				animate);
		}

		self.curs += 1;

		(sample, self.curs == self.audio.len())
	}

	pub fn transform(&mut self, run: fn(&mut f32, &Audio, usize, f32) -> (),
		range: (usize, usize)) -> ()
	{
		let transform = Transform {
			run: run,
			range: (range.0, (range.1 - range.0) as f32),
		};

		self.transforms.push(transform);
	}

	pub fn play(&self) -> () {
		
	}
}

impl<'a> PartialEq<Audio> for Stream<'a> {
	fn eq(&self, other: &Audio) -> bool {
		self.audio as *const _ == other as *const _
	}
}
