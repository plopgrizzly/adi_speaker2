/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "ffi/pulse_audio/context_create.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

use std::process;
use std::mem;
use std::ptr::null;
use std::ffi::CString;
use super::{ LazyPointer, SampleSpec, Format, BUFFER_LEN };

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
	userdata: LazyPointer,
	io_new: LazyPointer,
	io_enable: LazyPointer,
	io_free: LazyPointer,
	io_set_destroy: LazyPointer,
	time_new: LazyPointer,
	time_restart: LazyPointer,
	time_free: LazyPointer,
	time_set_destroy: LazyPointer,
	defer_new: LazyPointer,
	defer_enable: LazyPointer,
	defer_free: LazyPointer,
	defer_set_destroy: LazyPointer,
	quit: unsafe extern "C" fn(a: *mut MainLoopApi, retval: i32) -> (),
}

#[repr(C)]
struct Volume {
	channels: u8,
	values: [u32; 32]
}

#[repr(C)]
pub struct Context {
	api: *mut MainLoopApi,
	pub data: *const u8,
	pub left: isize,
	pub used: isize,
	pub context: LazyPointer,
}

extern {
	fn pa_mainloop_get_api(m: LazyPointer) -> *mut MainLoopApi;
	fn pa_context_new(mainloop: *mut MainLoopApi, name: *const i8)
		-> LazyPointer;
	fn pa_context_set_state_callback(
		c: LazyPointer,
		cb: extern "C" fn(c: LazyPointer, userdata: *mut Context) -> (),
		userdata: *mut Context) -> ();
	fn pa_context_get_state(c: LazyPointer) -> ContextState;
	fn pa_context_connect(c: LazyPointer, server: LazyPointer,
		flags: ContextFlags, api: LazyPointer) -> i32;
	fn pa_stream_new(c: LazyPointer, name: *const i8, ss: *const SampleSpec,
		map: LazyPointer) -> LazyPointer;

	fn pa_stream_set_state_callback(s: LazyPointer,
		cb: extern "C" fn(s: LazyPointer, userdata: LazyPointer) -> (),
		userdata: LazyPointer) -> ();
	fn pa_stream_set_write_callback(p: LazyPointer,
		cb: extern "C" fn(
			p: LazyPointer,
			nbytes: usize,
			userdata: *mut Context) -> (),
		userdata: *mut Context) -> ();
	fn pa_stream_connect_playback(s: LazyPointer, dev: LazyPointer,
		attr: LazyPointer, flags: StreamFlags, volume: LazyPointer,
		sync_stream: LazyPointer) -> i32;
	// cvolume
	fn pa_cvolume_set(a: *mut Volume, channels: u32, v: u32) -> LazyPointer;

	fn pa_stream_get_state(p: LazyPointer) -> StreamState;
	fn pa_stream_write(p: LazyPointer, data: *const i16, nbytes: usize,
		free_cb: unsafe extern "C" fn(p: LazyPointer) -> (), offset: i64,
		seek: SeekMode) -> ();

	fn pa_xstrdup(s: *const i8) -> *const i8;
	fn pa_xmalloc(l: usize) -> *const i16;
	fn pa_xfree(p: LazyPointer) -> ();
	fn memcpy(dest: *const i16, src: *const u8, n: usize) -> LazyPointer;
	fn malloc(size: usize) -> *mut Context;
}

const ADISPEAKER_SS : SampleSpec = SampleSpec {
	format: Format::SampleS16LE,
	rate: 44100,
	channels: 2,
};

macro_rules! veci16_placeholder {
	() => {
		&0 as *const _ as *const Vec<i16>
	};
}

macro_rules! u8_placeholder {
	() => {
		&0 as *const _ as *const u8
	};
}

// pub static mut ADISPEAKER_BUFFER : *const Vec<i16> = veci16_placeholder!();

pub static mut ADISPEAKER_BUFFER : [i16; BUFFER_LEN] = [0; BUFFER_LEN];//*const u8 = u8_placeholder!();

/* This is called whenever new data may be written to the stream */
extern "C" fn stream_write_callback(s: LazyPointer, length: usize,
	context: *mut Context)
{
	let left = unsafe { (*context).left };

	if left <= length as isize {
		panic!("Buffer Underrun!");
	}

	let out_data = unsafe { pa_xmalloc(length) };

	unsafe {
		memcpy(out_data, /*&ADISPEAKER_BUFFER[0]*/(*context).data, length);

		(*context).left -= length as isize;
		(*context).used += length as isize;
	}

	unsafe {
		pa_stream_write(s, out_data, length, pa_xfree, 0,
			SeekMode::Relative);
	}
}

/* This routine is called whenever the stream state changes */
extern "C" fn stream_state_callback(s: LazyPointer, _: LazyPointer) {
	match unsafe { pa_stream_get_state(s) } {
		StreamState::Creating | StreamState::Terminated
			| StreamState::Ready => { /* do nothing */ }
		_ => panic!("Stream error")
	}
}

extern "C" fn context_state_callback(c: LazyPointer, context: *mut Context) {
	let volume = 65536; // TODO

	match unsafe { pa_context_get_state(c) } {
		ContextState::Connecting | ContextState::Authorizing
			| ContextState::SettingName => { /*Do nothing*/ },
		ContextState::Ready => {
			println!("Ready!");

			let name = CString::new("coolnamy").unwrap();
			let name = unsafe { pa_xstrdup(name.as_ptr()) };

			let stream = unsafe {
				pa_stream_new(c, name, &ADISPEAKER_SS, 0)
			};

			if stream == 0 {
				panic!("Couldn't create stream!");
			}

			let mut cv = Volume {
				channels: 0,
				values: [0; 32]
			};

			unsafe {
				pa_stream_set_state_callback(stream,
					stream_state_callback, 0);
				pa_stream_set_write_callback(stream,
					stream_write_callback, context);
				pa_stream_connect_playback(stream, 0, 0,
					StreamFlags::NoFlags,
					pa_cvolume_set(&mut cv, 2, volume), 0);
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

pub fn context_create(connection: LazyPointer, name: &str) -> *mut Context {
	let rtn = unsafe {
		malloc(mem::size_of::<Context>())
	};

	let name = CString::new(name).unwrap();
	let name = unsafe { pa_xstrdup(name.as_ptr()) };

	println!("Creating Context");

	unsafe {
		println!("Mainloop");

		(*rtn).api = pa_mainloop_get_api(connection);

		println!("CTX");

		(*rtn).context = pa_context_new((*rtn).api, name);
		println!("Made contxt {}", (*rtn).context);

		pa_context_set_state_callback((*rtn).context,
			context_state_callback, rtn);

		(*rtn).left = 0;
	};

	println!("Connecting to Context");

	if unsafe {
		pa_context_connect((*rtn).context, 0, ContextFlags::NoFlags, 0)
	} < 0 {
		panic!("Couldn't connect to context!");
	}

	println!("Connected to Context");

	rtn
}
