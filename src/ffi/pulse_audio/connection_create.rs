/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "ffi/pulse_audio/connection_create.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

use super::LazyPointer;

extern {
	fn pa_mainloop_new() -> LazyPointer;
}

pub fn connection_create() -> LazyPointer {
	let connection = unsafe {
		pa_mainloop_new()
	};

	if connection == 0 {
		panic!("Couldn't connect to speakers!");
	}

	connection
}
