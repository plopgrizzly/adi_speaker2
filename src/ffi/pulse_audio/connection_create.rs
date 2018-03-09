// ffi/pulse_audio/connection_create.rs -- Aldaron's Device Interface / Speaker
// Copyright (c) 2017  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use ami::void_pointer::*;

extern {
	fn pa_mainloop_new() -> VoidPointer;
}

pub fn connection_create() -> VoidPointer {
	let connection = unsafe {
		pa_mainloop_new()
	};

	if connection == NULL {
		panic!("Couldn't connect to speakers!");
	}

	connection
}
