/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "audio.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

static mut UUID : usize = 0;

pub struct Audio {
	buffer: Vec<u8>,
	uuid: usize,
}

pub trait AudioReader {
	fn read(&self) -> Vec<u8>;
	fn uuid(&self) -> usize;
}

impl AudioReader for Audio {
	fn read(&self) -> Vec<u8> {
		self.buffer.clone()
	}

	fn uuid(&self) -> usize {
		self.uuid
	}
}

fn gen_uuid() -> usize {
	unsafe {
		let uuid = UUID;
		UUID += 1;
		uuid
 	}
}

impl Audio {
	pub fn create(bytes: &'static [u8]) -> Audio {
		let uuid = gen_uuid();
		let mut buffer = Vec::new();

		buffer.extend_from_slice(bytes);

		Audio { buffer: buffer, uuid: uuid }
	}
}
