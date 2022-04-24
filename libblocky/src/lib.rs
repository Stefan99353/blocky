#[macro_use]
extern crate log;

#[allow(dead_code)]
mod consts;
mod either;
pub mod error;
#[cfg(feature = "gobject")]
pub mod gobject;
#[cfg(feature = "helpers")]
pub mod helpers;
pub mod instance;
mod os;
mod profile;

pub use instance::Instance;
pub use profile::Profile;

fn test() {
    let _x = instance::InstanceBuilder::default()
        .name("asdf".to_string())
        .build();
}
