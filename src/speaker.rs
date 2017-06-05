/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "speaker.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

use std::ptr::copy;
use std::ops::Range;

use ffi;
use Audio;
use Mixer;
use HZ;
use std::i16;
use std::mem;

const HZF : f32 = HZ as f32;
const BUFFER_LEN : usize = HZ * 2 * 10; // 2 channels, 10 second buffer.
const SBUFFER_LEN : isize = BUFFER_LEN as isize;
const WAVE_MAX : f32 = i16::MAX as f32;

/** The computer/phone speakers or headphones. */
pub struct Speaker<'a> {
	speaker: ffi::Speaker<'a>,
//	buffer: [i16; BUFFER_LEN],
}

impl<'a> Speaker<'a> {
	/** Connect to the speaker.
	    # Example
	    ```
	    use Speaker;

	    let speaker = Speaker::create();
	    ```
	*/
	pub fn create() -> Speaker<'a> {
		let mixer = Mixer::create(Mixer::mixer_blend);
		let mut buffer = Vec::new();

		buffer.resize(BUFFER_LEN, 0i16);

		Speaker {
			speaker: ffi::Speaker::create(mixer),
		}
	}

	/** Play `audio` on the speaker, starting `seconds_in` seconds in and
	    fading in for `fade` seconds. */
	pub fn play(&mut self, audio: &'a Audio, seconds_in: f32, fade: f32) {
		self.speaker.add_stream(audio);
	}

	/** Stop the playback of `audio`, fading out for fade seconds */
	pub fn stop(&self, audio: &Audio, fade: f32) -> f32 {
		0.0
	}

	/** Returns true if `audio` is being played, and false otherwise */
	pub fn is_playing(&self, audio: &'a Audio) -> bool {
		self.speaker.is_playing(audio)
	}

	/** Update the speaker's audio buffer **/
	pub fn update(&mut self) -> () {
		self.speaker.update();
	}
}
