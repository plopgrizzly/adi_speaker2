// audio.rs -- Aldaron's Device Interface / Speaker
// Copyright (c) 2017  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

// use core::ops::{ Index, IndexMut };
use std::ops::{ Index, IndexMut };
use std::slice;
use std::i16;

const WAVE_MAX : f32 = i16::MAX as f32;

pub struct Audio(Vec<i16>, isize);

impl Audio {
	pub fn create(bytes: &'static [u8]) -> Audio {
		let nsamples = bytes.len() / 2;
		let samples = bytes as *const _ as *const i16;
		let data = unsafe { slice::from_raw_parts(samples, nsamples) };

		let mut buffer = Vec::with_capacity(nsamples);

		unsafe {
			buffer.set_len(nsamples);
		}
		buffer[0..nsamples].clone_from_slice(data);

		Audio(buffer, nsamples as isize)
	}

	pub fn len(&self) -> isize {
		self.1
	}

	/// Sample the audio at index `i`
	pub fn sample(&self, i: isize) -> f32 {
		if i >= 0 {
			self[i as usize] as f32 / WAVE_MAX
		} else {
			0.0
		}
	}
}

impl Index<usize> for Audio {
	type Output = i16;

	fn index(&self, i: usize) -> &Self::Output {
		&self.0[i]
	}
}

impl IndexMut<usize> for Audio {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		&mut self.0[i]
	}
}
