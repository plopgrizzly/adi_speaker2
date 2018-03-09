// ffi/pulse_audio/context_create.rs -- Aldaron's Device Interface / Speaker
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use libc;
use ami::void_pointer::*;
use Mixer;
use HZ;

use std::mem;
use std::ffi::CString;
use super::{ SampleSpec, Format };

#[repr(C)]
enum ContextFlags {
	NoFlags = 0,
}

#[repr(C)]
enum StreamFlags {
	NoFlags = 0,
}

#[allow(dead_code)]
#[repr(C)]
enum StreamState {
	Unconnected = 0,
	Creating = 1,
	Ready = 2,
	Failed = 3,
	Terminated = 4,
}

#[allow(dead_code)]
#[repr(C)]
enum ContextState {
	Unconnected = 0,
	Connecting = 1,
	Authorizing = 2,
	SettingName = 3,
	Ready = 4,
	Failed = 5,
	Terminated = 6,
}

#[repr(C)]
enum SeekMode {
	Relative = 0,
}

#[repr(C)]
pub struct MainLoopApi {
	userdata: VoidPointer,
	io_new: VoidPointer,
	io_enable: VoidPointer,
	io_free: VoidPointer,
	io_set_destroy: VoidPointer,
	time_new: VoidPointer,
	time_restart: VoidPointer,
	time_free: VoidPointer,
	time_set_destroy: VoidPointer,
	defer_new: VoidPointer,
	defer_enable: VoidPointer,
	defer_free: VoidPointer,
	defer_set_destroy: VoidPointer,
	quit: unsafe extern "C" fn(a: *mut MainLoopApi, retval: i32) -> (),
}

#[repr(C)]
struct Volume {
	channels: u8,
	values: [u32; 32]
}


#[repr(C)]
pub struct Context<'a> {
	api: *mut MainLoopApi,
	pub context: VoidPointer,
	pub mixer: Mixer<'a>,
}

// Allow improper_ctypes for Context because the struct's use is for callbacks.
#[allow(improper_ctypes)]
extern {
	fn pa_context_set_state_callback(
		c: VoidPointer,
		cb: extern "C" fn(c: VoidPointer, userdata: *mut Context) -> (),
		userdata: *mut Context) -> ();
	fn pa_stream_set_write_callback(p: VoidPointer,
		cb: extern "C" fn(
			p: VoidPointer,
			nbytes: usize,
			userdata: *mut Context) -> (),
		userdata: *mut Context) -> ();
}

extern {
	fn pa_mainloop_get_api(m: VoidPointer) -> *mut MainLoopApi;
	fn pa_context_new(mainloop: *mut MainLoopApi, name: *const i8)
		-> VoidPointer;
	fn pa_context_get_state(c: VoidPointer) -> ContextState;
	fn pa_context_connect(c: VoidPointer, server: VoidPointer,
		flags: ContextFlags, api: VoidPointer) -> i32;
	fn pa_stream_new(c: VoidPointer, name: *const i8, ss: *const SampleSpec,
		map: VoidPointer) -> VoidPointer;

	fn pa_stream_set_state_callback(s: VoidPointer,
		cb: extern "C" fn(s: VoidPointer, userdata: VoidPointer) -> (),
		userdata: VoidPointer) -> ();
	fn pa_stream_connect_playback(s: VoidPointer, dev: VoidPointer,
		attr: VoidPointer, flags: StreamFlags, volume: VoidPointer,
		sync_stream: VoidPointer) -> i32;
	// cvolume
	fn pa_cvolume_set(a: *mut Volume, channels: u32, v: u32) -> VoidPointer;

	fn pa_stream_get_state(p: VoidPointer) -> StreamState;
	fn pa_stream_write(p: VoidPointer, data: *const i16, nbytes: usize,
		free_cb: unsafe extern "C" fn(p: VoidPointer) -> (), offset: i64,
		seek: SeekMode) -> ();

	fn pa_xstrdup(s: *const i8) -> *const i8;
	fn pa_xmalloc(l: usize) -> *mut i16;
	fn pa_xfree(p: VoidPointer) -> ();
}

const ADISPEAKER_SS : SampleSpec = SampleSpec {
	format: Format::SampleS16LE,
	rate: HZ,
	channels: 2,
};

/* This is called whenever new data may be written to the stream */
extern "C" fn stream_write_callback(s: VoidPointer, length: usize,
	context: *mut Context)
{
//	let left = unsafe { (*context).left };
//
//	if left <= length as isize {
//		panic!("Buffer Underrun!");
//	}

	let out_data = unsafe { pa_xmalloc(length) };

	for i in 0..(length/2) {
		unsafe {
			(*out_data.wrapping_offset(i as isize)) =
				(*context).mixer.update();
		}
	}

//	unsafe {
//		memcpy(out_data, /*&ADISPEAKER_BUFFER[0]*/(*context).data, length);
//
//		(*context).left -= length as isize;
//		(*context).used += length as isize;
//	}

	unsafe {
		pa_stream_write(s, out_data, length, pa_xfree, 0,
			SeekMode::Relative);
	}
}

/* This routine is called whenever the stream state changes */
extern "C" fn stream_state_callback(s: VoidPointer, _: VoidPointer) {
	match unsafe { pa_stream_get_state(s) } {
		StreamState::Creating | StreamState::Terminated
			| StreamState::Ready => { /* do nothing */ }
		_ => panic!("Stream error")
	}
}

extern "C" fn context_state_callback(c: VoidPointer, context: *mut Context) {
	let volume = 65536; // TODO

	match unsafe { pa_context_get_state(c) } {
		ContextState::Connecting | ContextState::Authorizing
			| ContextState::SettingName => { /*Do nothing*/ },
		ContextState::Ready => {
			println!("Ready!");

			let name = CString::new("coolnamy").unwrap();
			let name = unsafe { pa_xstrdup(name.as_ptr()) };

			let stream = unsafe {
				pa_stream_new(c, name, &ADISPEAKER_SS, NULL)
			};

			if stream == NULL {
				panic!("Couldn't create stream!");
			}

			let mut cv = Volume {
				channels: 0,
				values: [0; 32]
			};

			unsafe {
				pa_stream_set_state_callback(stream,
					stream_state_callback, NULL);
				pa_stream_set_write_callback(stream,
					stream_write_callback, context);
				pa_stream_connect_playback(stream, NULL, NULL,
					StreamFlags::NoFlags,
					pa_cvolume_set(&mut cv, 2, volume), NULL);
			}
		},
		ContextState::Terminated => unsafe {
			println!("Terminate");
			((*(*context).api).quit)((*context).api, 0)
		},
		_ => {
			panic!("Connection Failed!");
		},
	}
}

pub fn context_create<'a>(connection: VoidPointer, name: &str, mixer: Mixer<'a>)
	-> *mut Context<'a>
{
	let rtn : *mut Context = unsafe {
		mem::transmute(libc::malloc(mem::size_of::<Context>()))
	};

	let name = CString::new(name).unwrap();
	let name = unsafe { pa_xstrdup(name.as_ptr()) };

	println!("Creating Context");

	unsafe {
		(*rtn).mixer = mixer;

		println!("Mainloop");

		(*rtn).api = pa_mainloop_get_api(connection);

		println!("CTX");

		(*rtn).context = pa_context_new((*rtn).api, name);
		println!("Made contxt {}", (*rtn).context);

		pa_context_set_state_callback((*rtn).context,
			context_state_callback, rtn);
	};

	println!("Connecting to Context");

	if unsafe {
		pa_context_connect((*rtn).context, NULL, ContextFlags::NoFlags,
			NULL)
	} < 0 {
		panic!("Couldn't connect to context!");
	}

	println!("Connected to Context");

	rtn
}
