/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "ffi/pulse_audio/context_update.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

use super::LazyPointer;

extern {
	fn pa_mainloop_iterate(m: LazyPointer, block: i32, retval: *mut i8)
		-> i32;
}

pub fn context_update(m: LazyPointer) {
	let mut ret = 1;

	unsafe {
		pa_mainloop_iterate(m, 0, &mut ret);
	}
}
