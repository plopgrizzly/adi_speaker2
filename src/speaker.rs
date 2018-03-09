// speaker.rs -- Aldaron's Device Interface / Speaker
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use ffi;
use Audio;
use Mixer;
use Stream;
use HZ;

/// A struct that's returned from `Speaker::play()`, for applying transforms to
/// the audio when it is played.
pub struct Settings;

/// The computer/phone speakers or headphones.
pub struct Speaker<'a> {
	speaker: ffi::Speaker<'a>
}

impl<'a> Speaker<'a> {
	/// Connect to the speaker.
	/// # Example
	/// ```
	/// let speaker = Speaker::create();
	/// ```
	pub fn create() -> Speaker<'a> {
		let mixer = Mixer::create(Mixer::mixer_blend);
		let speaker = ffi::Speaker::create(mixer);

		Speaker { speaker: speaker }
	}

	/// Play `audio` on the speaker, starting `seconds_in` seconds in and
	/// fading in for `fade` seconds.
	pub fn play(&mut self, audio: &'a Audio) -> Settings {
		self.speaker.add_stream(audio);
		Settings
	}

	/// Stop the playback of `audio`, fading out for fade seconds
	pub fn stop(&self, audio: &Audio, fade: f32) -> f32 {
		0.0
	}

	/// Returns true if `audio` is being played, and false otherwise
	pub fn is_playing(&self, audio: &'a Audio) -> bool {
		self.speaker.is_playing(audio)
	}

	/// Update the speaker's audio buffer
	pub fn update(&mut self) -> () {
		self.speaker.update();
	}
}

impl Settings {
	/// Apply a transform to the audio that `Speaker::play()` was last
	/// called on.  Apply it on range defined by `range`, in seconds.
	pub fn transform<F>(self, speaker: &mut Speaker,
		run: F,
		range: (f32, f32)) -> Settings where
		F: Fn(&mut f32, &Audio, isize, f32) -> () + 'static
	{
		speaker.speaker.transform(Box::new(run), range);

		self
	}

	/// Fade in audio that `Speaker::play()` was last called on.  Apply the
	/// fade in on range defined by `range`, in seconds.
	pub fn fade_in(self, speaker: &mut Speaker, range: (f32, f32))
		-> Settings
	{
		self.transform(speaker, |out:&mut f32, _, _, fade| {
			*out *= fade;
		}, range)
	}

	/// Fade out audio that `Speaker::play()` was last called on.  Apply the
	/// fade out on range defined by `range`, in seconds.
	pub fn fade_out(self, speaker: &mut Speaker, range: (f32, f32))
		-> Settings
	{
		self.transform(speaker, |out:&mut f32, _, i, fade| {
			*out *= 1.0 - fade;
		}, range)
	}

	/// Set the volume of the audio that `Speaker::play()` was last called
	/// on.  Apply the volume on range defined by `range`, in seconds.
	/// `vol` is how loud the the audio should be played, 1.0 being no
	/// change, and 0.0 being silent, and 2.0 being twice as loud (beware
	/// clipping).
	pub fn volume(self, speaker: &mut Speaker, range: (f32, f32), vol: f32)
		-> Settings
	{
		self.transform(speaker, move|out:&mut f32, _, _, _| {
			*out *= vol;
		}, range)
	}

	/// Echo the audio that `Speaker::play()` was last called on.  Apply the
	/// echo on range defined by `range`, in seconds. `s` is a tuple that is
	/// defined by `(volume, offset)`, volume is how loud the echo is, 1.0
	/// being just as loud as the original audio, offset is how many seconds
	/// earlier in the audio the echo is being sampled from.
	pub fn echo(self, speaker: &mut Speaker, range: (f32,f32), s: (f32,f32))
		-> Settings
	{
		self.transform(speaker, move|out:&mut f32, audio:&Audio, i, _| {
			*out += s.0 * audio.sample(i - (HZ as isize));
		}, range)
	}
}
