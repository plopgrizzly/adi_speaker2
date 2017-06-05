/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "audio.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

use ami::void_pointer::*;
// use core::ops::{ Index, IndexMut };
use std::ops::{ Index, IndexMut };
use std::slice;

pub struct Audio(Vec<i16>);

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

		Audio(buffer)
	}

	pub fn len(&self) -> usize {
		self.0.len()
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
