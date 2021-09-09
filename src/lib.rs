//! # Rust bindings for **libostree**
//!
//! [libostree](https://ostree.readthedocs.io) is both a shared library and suite of command line
//! tools that combines a "git-like" model for committing and downloading bootable filesystem trees,
//! along with a layer for deploying them and managing the bootloader configuration.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![deny(unused_must_use)]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

// Re-export our dependencies.  See https://gtk-rs.org/blog/2021/06/22/new-release.html
// "Dependencies are re-exported".  Users will need e.g. `gio::File`, so this avoids
// them needing to update matching versions.
pub use ffi;
pub use gio;
pub use glib;

/// Useful with `Repo::open_at()`.
pub use libc::AT_FDCWD;

// code generated by gir
#[rustfmt::skip]
#[allow(clippy::all)]
#[allow(unused_imports)]
#[allow(missing_docs)]
mod auto;
pub use crate::auto::functions::*;
pub use crate::auto::*;

// handwritten code
mod checksum;
pub use crate::checksum::*;
mod core;
pub use crate::core::*;

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

/// Prelude, intended for glob imports.
pub mod prelude {
    pub use crate::auto::traits::*;
    // See "Re-export dependencies above".
    #[doc(hidden)]
    pub use gio::prelude::*;
    #[doc(hidden)]
    pub use glib::prelude::*;
}
