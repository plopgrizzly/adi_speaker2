// ffi/pulse_audio/sample_spec_create.rs -- Aldaron's Device Interface / Speaker
// Copyright (c) 2017  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use std::fmt;

#[repr(C)]
pub enum Format {
	SampleS16LE = 3,
}

impl fmt::Display for Format {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Format::SampleS16LE => write!(f, "s16le"),
		}
	}
}

#[repr(C)]
pub struct SampleSpec {
	pub format: Format,
	pub rate: u32,
	pub channels: u8,
}

#[link(name = "pulse")]
extern {
	fn pa_sample_spec_valid(spec: *const SampleSpec) -> i32;
}

pub fn sample_spec_create() {
	let spec = SampleSpec {
		format: Format::SampleS16LE,
		rate: 44100,
		channels: 2,
	};

	if unsafe {
		pa_sample_spec_valid(&spec)
	} == 0 {
		panic!("Couldn't use sample spec!");
	}

	println!("adi_speaker: Backend: PulseAudio");
	println!("adi_speaker: Format: {}", spec.format);
	println!("adi_speaker: Channels: {}", spec.channels);
	println!("adi_speaker: Hz: {}", spec.rate);
}
