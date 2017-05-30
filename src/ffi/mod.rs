/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "ffi/mod.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

use HZ;

const BUFFER_LEN : usize = HZ * 2 * 10; // 2 channels, 10 second buffer.

#[repr(C)]
struct Buffer {
	data: [i16; BUFFER_LEN],
}

type LazyPointer = usize;

mod pulse_audio;

pub use self::pulse_audio::Speaker;
