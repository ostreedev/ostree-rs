//! # Rust bindings for [libostree](https://ostree.readthedocs.io)
//!
//! libostree is both a shared library and suite of command line tools that combines
//! a "git-like" model for committing and downloading bootable filesystem trees,
//! along with a layer for deploying them and managing the bootloader configuration.

#![doc(html_root_url = "https://fkrull.gitlab.io/ostree-rs")]

extern crate gio_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate ostree_sys;
#[macro_use]
extern crate glib;
extern crate gio;
extern crate libc;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

use glib::Error;

// code generated by gir
#[allow(unused_imports)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod auto;
pub use crate::auto::functions::*;
pub use crate::auto::*;

// handwritten code
mod object_name;
mod repo;
pub use crate::object_name::*;
pub use crate::repo::*;

// tests
#[cfg(test)]
mod tests;

// prelude
pub mod prelude {
    pub use crate::auto::traits::*;
}
