/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "speaker.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

use ffi;
use Audio;
use audio::AudioReader;
use Mixer;
use HZ;
use std::i16;

const HZF : f32 = HZ as f32;
const BUFFER_LEN : usize = HZ * 2 * 10; // 2 channels, 10 second buffer.
const SBUFFER_LEN : isize = BUFFER_LEN as isize;
const WAVE_MAX : f32 = i16::MAX as f32;

struct Stream {
	data: *const i16,
	left: isize,
	curs: isize,
	uuid: usize,
//	size: isize,
}
impl Stream {
	fn sample(&mut self) -> (i16, bool) {
		let data = if self.left > 0 /*&& self.left <= self.size*/ {
			unsafe { *(self.data.offset(self.curs)) }
		} else {
			0
		};

		self.left -= 1;
		self.curs += 1;

		(data, self.left <= -SBUFFER_LEN / 2)
	}

	fn backtrack(&mut self) {
		self.left += SBUFFER_LEN;
		self.curs -= SBUFFER_LEN;
	}
}

/** The computer/phone speakers or headphones. */
pub struct Speaker {
	speaker: ffi::Speaker,
	buffer: Vec<i16>,
	streams: Vec<Stream>,
}

impl Speaker {
	/** Connect to the speaker.
	    # Example
	    ```
	    use Speaker;

	    let speaker = Speaker::create();
	    ```
	*/
	pub fn create() -> Speaker {
		let mut buffer = Vec::new();

		buffer.resize(BUFFER_LEN, 0i16);

		Speaker {
			speaker: ffi::Speaker::create(),
			buffer: buffer,
			streams: Vec::new(),
		}
	}

	/** Play `audio` on the speaker, starting `seconds_in` seconds in and
	    fading in for `fade` seconds. */
	pub fn play(&mut self, audio: &Audio, seconds_in: f32, fade: f32) {
		// 2 channels * 2 bytes in S16 = 4.0
		let samples_in = (seconds_in * 4.0 * HZF) as isize;
		let read = audio.read();

		self.streams.push(Stream {
			data: unsafe { read.as_ptr().offset(samples_in) }
				as *const _,
			left: (read.len() as isize) - samples_in,
			curs: 0,
			uuid: audio.uuid()
		});

		self.buffer.clear();
	}

	/** Stop the playback of `audio`, fading out for fade seconds */
	pub fn stop(&self, audio: &Audio, fade: f32) -> f32 {
		0.0
	}

	/** Returns true if `audio` is being played, and false otherwise */
	pub fn is_playing(&self, audio: &Audio) -> bool {
		for stream in &self.streams {
			if stream.uuid == audio.uuid() {
				return true;
			}
		}
		false
	}

	/** Update the speaker's audio buffer **/
	pub fn update(&mut self) {
		if self.buffer.is_empty() {
			for i in 0..BUFFER_LEN {
				self.sample();
			}
		}

		let used = self.speaker.update() / 2;

		let _: Vec<_> = self.buffer.drain(0..used as usize).collect();

		for i in 0..used {
			self.sample();
		}

		self.speaker.play(self.buffer.as_ptr() as *const _, SBUFFER_LEN); // TODO: move to create()
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

		self.buffer.push((Mixer::blend(samples) * WAVE_MAX) as i16);
	}
}
