/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "ffi/pulse_audio/destroy.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

use ami::void_pointer::*;

extern {
	fn pa_signal_done() -> ();
	fn pa_mainloop_free(m: VoidPointer) -> ();
}

pub fn connection(connection: VoidPointer) {
	unsafe {
		pa_signal_done();
		pa_mainloop_free(connection);
	}
}
