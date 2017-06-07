/*
 * adi_speaker - Aldaron's Device Interface
 * Speaker - "speaker.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

use ffi;
use Audio;
use Mixer;
use Stream;

pub struct Settings;

/** The computer/phone speakers or headphones. */
pub struct Speaker<'a> {
	speaker: ffi::Speaker<'a>
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
		let speaker = ffi::Speaker::create(mixer);

		Speaker { speaker: speaker }
	}

	/** Play `audio` on the speaker, starting `seconds_in` seconds in and
	    fading in for `fade` seconds. */
	pub fn play(&mut self, audio: &'a Audio) -> Settings {
		self.speaker.add_stream(audio);
		Settings
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

impl Settings {
	pub fn transform(self, speaker: &mut Speaker,
		run: Box<Fn(&mut f32, &Audio, usize, f32) -> ()>,
		range: (f32, f32)) -> Settings
	{
		speaker.speaker.transform(run, range);

		self
	}

	pub fn fade_in(self, speaker: &mut Speaker, range: (f32, f32))
		-> Settings
	{
		self.transform(speaker, Box::new(move |out, audio, index, animate| {
			*out = audio.sample(index) * animate;
		}), range)
	}

	pub fn fade_out(self, speaker: &mut Speaker, range: (f32, f32))
		-> Settings
	{
		self.transform(speaker, Box::new(move |out, audio, index, animate| {
			*out = audio.sample(index) * (1.0 - animate);
		}), range)
	}

	pub fn volume(self, speaker: &mut Speaker, range: (f32, f32), vol: f32)
		-> Settings
	{
		self.transform(speaker, Box::new(move |out, audio, index, _| {
			*out = audio.sample(index) * vol;
		}), range)
	}
}
