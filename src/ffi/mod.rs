// ffi/mod.rs -- Aldaron's Device Interface / Speaker
// Copyright (c) 2017  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

mod pulse_audio;

pub use self::pulse_audio::Speaker;
