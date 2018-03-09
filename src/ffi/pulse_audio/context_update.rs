// ffi/pulse_audio/context_update.rs -- Aldaron's Device Interface / Speaker
// Copyright (c) 2017  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use ami::void_pointer::*;

extern {
	fn pa_mainloop_iterate(m: VoidPointer, block: i32, retval: *mut i8)
		-> i32;
}

pub fn context_update(m: VoidPointer) {
	let mut ret = 1;

	unsafe {
		pa_mainloop_iterate(m, 0, &mut ret);
	}
}
