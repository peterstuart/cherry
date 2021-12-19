#![cfg_attr(not(test), no_std)]

//! # Examples
//!
//! The [examples][examples] use the [embedded-graphics simulator][simulator], so
//! follow the [setup instructions][setup] before running them.
//!
//! ```shell
//! cargo run --example container
//! cargo run --example text
//! ```
//!
//! [examples]: https://github.com/peterstuart/cherry/tree/main/examples
//! [simulator]: https://github.com/embedded-graphics/simulator
//! [setup]: https://github.com/embedded-graphics/simulator#setup

extern crate alloc;

pub mod widget;
