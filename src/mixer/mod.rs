// mixer/mod.rs -- Aldaron's Device Interface / Speaker
// Copyright (c) 2017  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

mod stream;

use HZ;
use Audio;
use std::i16;

pub use self::stream::Stream;

const WAVE_MAX : f32 = i16::MAX as f32;

pub struct Mixer<'a> {
	mixer: fn(samples: Vec<f32>) -> f32,
	streams: Vec<Stream<'a>>,
}
impl<'a> Mixer<'a> {
	/// Create a `Mixer` object.
	pub fn create(mixer: fn(samples: Vec<f32>) -> f32)
		-> Mixer<'a>
	{
		Mixer { mixer: mixer, streams: Vec::new() }
	}

	/// Add a stream to the `Mixer`.
	pub fn add_stream(&mut self, audio: &'a Audio) {
		self.streams.push(Stream::create(audio));
	}

	/// Add a transform to the last created stream.
	pub fn transform(&mut self,
		run: Box<Fn(&mut f32, &Audio, isize, f32) -> ()>,
		range: (f32, f32))
	{
		let hz = HZ as f32;
		let range = ((range.0 * hz) as usize, (range.1 * hz) as usize);

		self.streams.last_mut().unwrap().transform(run, range)
	}

	/// Returns true if audio is on one of the streams.
	pub fn is_playing(&self, audio: &'a Audio) -> bool {
		for stream in &self.streams {
			if stream == audio {
				return true;
			}
		}
		false
	}

	/// Mix the next sample.
	pub fn update(&mut self) -> i16 {
		let mut vec_samples = Vec::with_capacity(self.streams.len());
		let mut vec_expires = Vec::with_capacity(0);
		let mut i = 0;

		// Get a sample from each stream and put it in a vector.
		for stream in &mut self.streams {
			let (sample, expired) = stream.update();

			if expired {
				vec_expires.push(i);
			}

			vec_samples.push(sample);

			i += 1;
		}

		// If any sounds have finished playing, remove their streams.
		if vec_expires.is_empty() == false {
			vec_expires.reverse();

			for i in vec_expires {
				self.streams.remove(i);
			}
		}

		((self.mixer)(vec_samples) * WAVE_MAX) as i16
	}

	/// Mix a vector of samples into one sample.
	pub fn mixer_blend(samples: Vec<f32>) -> f32 {
		if samples.is_empty() {
			return 0.0;
		} else if samples.len() == 1 {
			return samples[0];
		}

//		let mut z = (samples[0] + 1.0) / 2.0; // 0.0 - 1.0
		let mut z = samples[0];

		for i in 1..samples.len() {
			let y = samples[i];

			z = (z + y) - (z * y);
		}

		// Algorithm adopted from
		// http://www.vttoth.com/CMS/index.php/technical-notes/68
/*		for i in 1..samples.len() {
			let y = samples[i];

			if z < 0.5 && y < 0.5 {
				z = 2.0 * z * y;
			} else {
				z = (2.0 * (z + y)) - (2.0 * (z * y)) - 1.0;
			}
		}*/

//		(z * 2.0) - 1.0 // -1.0 - 1.0
		z
	}
}
