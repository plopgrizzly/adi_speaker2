/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "ffi/pulse_audio/destroy.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

use super::LazyPointer;

extern {
	fn pa_signal_done() -> ();
	fn pa_mainloop_free(m: LazyPointer) -> ();
}

pub fn connection(connection: LazyPointer) {
	unsafe {
		pa_signal_done();
		pa_mainloop_free(connection);
	}
}
