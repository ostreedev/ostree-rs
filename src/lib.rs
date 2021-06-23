//! # Rust bindings for **libostree**
//!
//! [libostree](https://ostree.readthedocs.io) is both a shared library and suite of command line
//! tools that combines a "git-like" model for committing and downloading bootable filesystem trees,
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
extern crate hex;
extern crate once_cell;
extern crate radix64;
extern crate thiserror;

// code generated by gir
#[rustfmt::skip]
#[allow(clippy::all)]
#[allow(unused_imports)]
mod auto;
pub use crate::auto::functions::*;
pub use crate::auto::*;

// handwritten code
mod checksum;
pub use crate::checksum::*;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
mod collection_ref;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub use crate::collection_ref::*;
mod functions;
pub use crate::functions::*;
#[cfg(any(feature = "v2019_3", feature = "dox"))]
mod kernel_args;
#[cfg(any(feature = "v2019_3", feature = "dox"))]
pub use crate::kernel_args::*;
mod object_name;
pub use crate::object_name::*;
mod repo;
pub use crate::repo::*;
#[cfg(any(feature = "v2016_8", feature = "dox"))]
mod repo_checkout_at_options;
#[cfg(any(feature = "v2016_8", feature = "dox"))]
pub use crate::repo_checkout_at_options::*;
mod repo_transaction_stats;
pub use repo_transaction_stats::RepoTransactionStats;
mod se_policy;
pub use crate::se_policy::*;
#[cfg(any(feature = "v2020_1", feature = "dox"))]
mod commit_sizes_entry;
#[cfg(any(feature = "v2020_1", feature = "dox"))]
pub use crate::commit_sizes_entry::*;
#[cfg(any(feature = "v2017_4", feature = "dox"))]
mod sysroot_write_deployments_opts;
#[cfg(any(feature = "v2017_4", feature = "dox"))]
pub use crate::sysroot_write_deployments_opts::*;
#[cfg(any(feature = "v2020_7", feature = "dox"))]
mod sysroot_deploy_tree_opts;
#[cfg(any(feature = "v2020_7", feature = "dox"))]
pub use crate::sysroot_deploy_tree_opts::SysrootDeployTreeOpts;

// tests
#[cfg(test)]
mod tests;

// prelude
pub mod prelude {
    pub use crate::auto::traits::*;
}
