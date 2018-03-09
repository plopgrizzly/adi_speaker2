// ffi/pulse_audio/destroy.rs -- Aldaron's Device Interface / Speaker
// Copyright (c) 2017  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

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
