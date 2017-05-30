/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "mixer.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

use std::i16;

const WAVE_MAX : f32 = i16::MAX as f32;

pub struct Mixer {
	mixer: fn(f32, f32, isize) -> f32,
	buffer: Vec<i16>,
}
impl Mixer {
	pub fn create(mixer: fn(f32, f32, isize) -> f32) -> Mixer {
		let buffer = Vec::new();

		Mixer { mixer: mixer, buffer: buffer }
	}

	pub fn execute(&self, buffer: i16, input: i16, curs: isize) -> i16 {
		let buffer : f32 = ( buffer as f32 ) / WAVE_MAX;
		let input : f32 = ( input as f32 ) / WAVE_MAX;
		let output = (self.mixer)(buffer, input, curs);

		if output >= 1.0 {
			i16::MAX
		} else if output <= -1.0 {
			i16::MIN
		} else {
			(output * WAVE_MAX) as i16
		}
	}

	/** Mix a few samples into one. */
	pub fn blend(samples: Vec<f32>) -> f32 {
		if samples.is_empty() {
			return 0.0;
		} else if samples.len() == 1 {
			return samples[0];
		}

		let mut z = (samples[0] + 1.0) / 2.0; // 0.0 - 1.0

//		for i in 1..samples.len() {
//			let y = samples[i];
//
//			z = (z + y) - (z * y);
//		}

		// Algorithm adopted from
		// http://www.vttoth.com/CMS/index.php/technical-notes/68
		for i in 1..samples.len() {
			let y = samples[i];

			if z < 0.5 && y < 0.5 {
				z = 2.0 * z * y;
			} else {
				z = (2.0 * (z + y)) - (2.0 * (z * y)) - 1.0;
			}
		}

		(z * 2.0) - 1.0 // -1.0 - 1.0
	}
}
