/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "ffi/pulse_audio/mod.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

use super::{ LazyPointer, Buffer };

mod connection_create;
mod context_create;
mod context_update;
mod destroy;
mod sample_spec_create;

const NAME: &'static str = env!("CARGO_PKG_NAME");

use self::sample_spec_create::Format;
use self::sample_spec_create::SampleSpec;
impl SampleSpec {
	fn create() -> () {
		sample_spec_create::sample_spec_create()
	}
}

pub struct Connection { native: LazyPointer }
impl Connection {
	fn create() -> Connection {
		Connection { native: connection_create::connection_create() }
	}
}
impl Drop for Connection {
	fn drop(&mut self) -> () {
		destroy::connection(self.native);
	}
}

use self::context_create::Context;
impl Context {
	fn create(connection: &Connection) -> *mut Context {
		let c = connection.native;

		context_create::context_create(c, NAME)
	}
}

pub struct Speaker {
	connection: Connection,
	context: *mut Context,
	
}
impl Speaker {
	pub fn create() -> Speaker {
		SampleSpec::create();
		let connection = Connection::create();
		let context = Context::create(&connection);

		Speaker {
			connection: connection,
			context: context,
		}
	}

	pub fn play(&mut self, data: *const u8, left: isize) {
		unsafe {
			(*self.context).data = data;
			(*self.context).left = left;
			(*self.context).used = 0;
		}
	}

	pub fn update(&mut self) -> isize {
		context_update::context_update(self.connection.native);

		unsafe {
			let used = (*self.context).used;

			(*self.context).used = 0;

			used
		}
	}
}
