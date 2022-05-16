#[macro_use]
extern crate log;

#[allow(dead_code)]
mod consts;
mod either;
mod error;
mod os;
mod utils;

pub mod instance;
pub mod minecraft;
pub mod profile;

#[cfg(feature = "gobject")]
pub mod gobject;
