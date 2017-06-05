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

struct Stream<'a> {
	data: &'a Audio,
	left: isize,
	curs: isize,
//	size: isize,
}
impl<'a> Stream<'a> {
	fn sample(&mut self) -> (i16, bool) {
		let data = if (self.curs as usize) < self.data.len() {
			self.data[self.curs as usize]
		} else {
			0
		};

		self.left -= 1;
		self.curs += 1;

		(data, (self.curs as usize) >= self.data.len() + BUFFER_LEN)
	}

	fn backtrack(&mut self) {
		self.left += SBUFFER_LEN;
		self.curs -= SBUFFER_LEN;
	}
}

/** The computer/phone speakers or headphones. */
pub struct Speaker<'a> {
	speaker: ffi::Speaker,
//	buffer: [i16; BUFFER_LEN],
	buffer: Vec<i16>,
	streams: Vec<Stream<'a>>,
	cursor: usize,
	clear: bool,
}

// TODO
pub unsafe fn move_memory_unchecked<T: Copy>(data: &mut [T], from: Range<usize>, to: usize) {
    debug_assert!(from.start <= from.end);
    debug_assert!(from.end <= data.len());
    debug_assert!(to <= data.len() - (from.end - from.start));
    let ptr = data.as_mut_ptr();
    copy(ptr.offset(from.start as isize),
         ptr.offset(to as isize),
         from.end - from.start)
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
		let mut buffer = Vec::new();

		buffer.resize(BUFFER_LEN, 0i16);

		Speaker {
			speaker: ffi::Speaker::create(),
//			buffer: [0; BUFFER_LEN],
			buffer: buffer,
			streams: Vec::new(),
			cursor: 0,
			clear: false,
		}
	}

	/** Play `audio` on the speaker, starting `seconds_in` seconds in and
	    fading in for `fade` seconds. */
	pub fn play(&mut self, audio: &'a Audio, seconds_in: f32, fade: f32) {
		// 2 channels = 2.0
		let samples_in = (seconds_in * 2.0 * HZF) as isize;
		let size = audio.len() as isize;

		self.streams.push(Stream {
			data: audio,
			left: size - samples_in,
			curs: 0,
		});

		self.buffer.clear();
		self.clear = true;
	}

	/** Stop the playback of `audio`, fading out for fade seconds */
	pub fn stop(&self, audio: &Audio, fade: f32) -> f32 {
		0.0
	}

	/** Returns true if `audio` is being played, and false otherwise */
	pub fn is_playing(&self, audio: &Audio) -> bool {
		for stream in &self.streams {
			if stream.data as *const _ == audio as *const _ {
				return true;
			}
		}
		false
	}

	/** Update the speaker's audio buffer **/
	pub fn update(&mut self) {
//		println!("{} {}", self.buffer.is_empty(), self.clear);
//		if self.buffer.is_empty() != self.clear {
//			println!("{} {}", self.buffer.is_empty(), self.clear);
//			panic!("AH SHIT");
//		}

//		if self.buffer.is_empty() {
		if self.clear {
			self.cursor = 0;
			for i in 0..BUFFER_LEN {
				self.sample();
			}
			self.clear = false;
		}

		self.speaker.play(self.buffer.as_ptr() as *const _, SBUFFER_LEN); // TODO: move to create()

		let used = self.speaker.update() / 2;

//		unsafe {
//			move_memory_unchecked(&mut self.buffer, (used as usize)..BUFFER_LEN, 0);
//		}
		self.buffer.drain(0..used as usize);

		self.cursor = BUFFER_LEN - (used as usize);
		for i in 0..used {
			self.sample();
		}
	}

	fn sample(&mut self) {
		let mut samples = Vec::new();

		for i in 0..self.streams.len() {
			let (sample, expired) = self.streams[i].sample();

			if expired {
				println!("Expired");
				self.streams.remove(i);
			}

			samples.push((sample as f32) / WAVE_MAX);
		}

//		self.buffer[self.cursor] = (Mixer::blend(samples) * WAVE_MAX) as i16;
		self.buffer.push((Mixer::blend(samples) * WAVE_MAX) as i16);
	}
}
