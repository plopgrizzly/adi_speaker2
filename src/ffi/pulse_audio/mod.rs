// ffi/pulse_audio/mod.rs -- Aldaron's Device Interface / Speaker
// Copyright (c) 2017  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use ami::void_pointer::*;
use Mixer;
use Audio;
use Stream;

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

pub struct Connection { native: VoidPointer }
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
impl<'a> Context<'a> {
	fn create(connection: &Connection, mixer: Mixer<'a>) -> *mut Context<'a> {
		let c = connection.native;

		context_create::context_create(c, NAME, mixer)
	}
}

pub struct Speaker<'a> {
	connection: Connection,
	context: *mut Context<'a>,
	
}
impl<'a> Speaker<'a> {
	pub fn create(mixer: Mixer<'a>) -> Speaker<'a> {
		SampleSpec::create();
		let connection = Connection::create();
		let context = Context::create(&connection, mixer);

		Speaker {
			connection: connection,
			context: context,
		}
	}

	pub fn add_stream(&mut self, audio: &'a Audio) {
		unsafe {
			(*self.context).mixer.add_stream(audio)
		}
	}

	pub fn transform(&mut self,
		run: Box<Fn(&mut f32, &Audio, isize, f32) -> ()>,
		range: (f32, f32))
	{
		unsafe {
			(*self.context).mixer.transform(run, range)
		}
	}

	pub fn is_playing(&self, audio: &'a Audio) -> bool {
		unsafe {
			(*self.context).mixer.is_playing(audio)
		}
	}

	pub fn update(&mut self) -> () {
		context_update::context_update(self.connection.native);
	}
}
