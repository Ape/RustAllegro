// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_audio"]
#![crate_type = "lib"]

#![feature(libc)]
#![feature(optin_builtin_traits)]
#![feature(core)]

extern crate allegro;
extern crate allegro_audio_sys;
#[macro_use]
extern crate allegro_util;
extern crate libc;

pub use addon::*;
pub use stream::*;
pub use properties::*;
pub use sink::*;
pub use mixer::*;
pub use sample::*;

mod addon;
mod stream;
mod properties;
mod sink;
mod mixer;
mod sample;

mod internal;
