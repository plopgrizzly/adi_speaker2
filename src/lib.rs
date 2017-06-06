/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "lib.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

pub const VERSION : &'static str = "adi_speaker 0.1.0";

mod ffi;
mod speaker;
mod audio;
mod mixer;

pub use speaker::Speaker;
pub use audio::Audio;
pub use mixer::Mixer;

const HZ : u32 = 44100;

pub extern crate ami;
