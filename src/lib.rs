// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Creation of APK overlay for headless, stateless alpine sbc creation
#![warn(
    clippy::all,
    // clippy::restriction,
    // clippy::pedantic,
    // clippy::cargo
)]

pub mod commands;
pub mod config;
pub mod docker;
pub mod hardware;
