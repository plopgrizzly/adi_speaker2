// lib.rs -- Aldaron's Device Interface / Speaker
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

//! A platform-agnostic library for playing & making sound through speakers,
//! earbuds or headphones.

extern crate libc;
pub extern crate ami;

mod ffi;
mod speaker;
mod audio;
mod mixer;

pub use speaker::{ Speaker, Settings };
pub use audio::Audio;
pub use mixer::Mixer;
pub use mixer::Stream;

const HZ : u32 = 44100;
