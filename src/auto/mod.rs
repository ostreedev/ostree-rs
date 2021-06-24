// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod async_progress;
pub use self::async_progress::{AsyncProgress, AsyncProgressClass, NONE_ASYNC_PROGRESS};
pub use self::async_progress::AsyncProgressExt;

mod bootconfig_parser;
pub use self::bootconfig_parser::{BootconfigParser, BootconfigParserClass};

mod content_writer;
pub use self::content_writer::{ContentWriter, ContentWriterClass, NONE_CONTENT_WRITER};
pub use self::content_writer::ContentWriterExt;

mod deployment;
pub use self::deployment::{Deployment, DeploymentClass};

mod gpg_verify_result;
pub use self::gpg_verify_result::{GpgVerifyResult, GpgVerifyResultClass};

mod mutable_tree;
pub use self::mutable_tree::{MutableTree, MutableTreeClass, NONE_MUTABLE_TREE};
pub use self::mutable_tree::MutableTreeExt;

mod repo;
pub use self::repo::{Repo, RepoClass};

mod repo_file;
pub use self::repo_file::{RepoFile, RepoFileClass, NONE_REPO_FILE};
pub use self::repo_file::RepoFileExt;

#[cfg(any(feature = "v2018_6", feature = "dox"))]
mod repo_finder;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub use self::repo_finder::{RepoFinder, NONE_REPO_FINDER};
#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub use self::repo_finder::RepoFinderExt;

#[cfg(any(feature = "v2018_6", feature = "dox"))]
mod repo_finder_avahi;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub use self::repo_finder_avahi::{RepoFinderAvahi, RepoFinderAvahiClass, NONE_REPO_FINDER_AVAHI};
#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub use self::repo_finder_avahi::RepoFinderAvahiExt;

#[cfg(any(feature = "v2018_6", feature = "dox"))]
mod repo_finder_config;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub use self::repo_finder_config::{RepoFinderConfig, RepoFinderConfigClass, NONE_REPO_FINDER_CONFIG};

#[cfg(any(feature = "v2018_6", feature = "dox"))]
mod repo_finder_mount;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub use self::repo_finder_mount::{RepoFinderMount, RepoFinderMountClass, NONE_REPO_FINDER_MOUNT};
#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub use self::repo_finder_mount::RepoFinderMountExt;

#[cfg(any(feature = "v2018_6", feature = "dox"))]
mod repo_finder_override;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub use self::repo_finder_override::{RepoFinderOverride, RepoFinderOverrideClass, NONE_REPO_FINDER_OVERRIDE};
#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub use self::repo_finder_override::RepoFinderOverrideExt;

mod se_policy;
pub use self::se_policy::{SePolicy, SePolicyClass};

#[cfg(any(feature = "v2020_2", feature = "dox"))]
mod sign;
#[cfg(any(feature = "v2020_2", feature = "dox"))]
pub use self::sign::{Sign, NONE_SIGN};
#[cfg(any(feature = "v2020_2", feature = "dox"))]
pub use self::sign::SignExt;

mod sysroot;
pub use self::sysroot::{Sysroot, SysrootClass};

mod sysroot_upgrader;
pub use self::sysroot_upgrader::{SysrootUpgrader, SysrootUpgraderClass};

#[cfg(any(feature = "v2018_6", feature = "dox"))]
mod collection_ref;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub use self::collection_ref::CollectionRef;

#[cfg(any(feature = "v2020_1", feature = "dox"))]
mod commit_sizes_entry;
#[cfg(any(feature = "v2020_1", feature = "dox"))]
pub use self::commit_sizes_entry::CommitSizesEntry;

mod diff_item;
pub use self::diff_item::DiffItem;

#[cfg(any(feature = "v2018_6", feature = "dox"))]
mod remote;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub use self::remote::Remote;

mod repo_commit_modifier;
pub use self::repo_commit_modifier::RepoCommitModifier;

mod repo_dev_ino_cache;
pub use self::repo_dev_ino_cache::RepoDevInoCache;

#[cfg(any(feature = "v2018_6", feature = "dox"))]
mod repo_finder_result;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub use self::repo_finder_result::RepoFinderResult;

mod enums;
pub use self::enums::DeploymentUnlockedState;
pub use self::enums::GpgSignatureAttr;
pub use self::enums::ObjectType;
#[cfg(any(feature = "v2018_2", feature = "dox"))]
pub use self::enums::RepoCheckoutFilterResult;
pub use self::enums::RepoCheckoutMode;
pub use self::enums::RepoCheckoutOverwriteMode;
pub use self::enums::RepoCommitFilterResult;
pub use self::enums::RepoCommitIterResult;
pub use self::enums::RepoMode;
pub use self::enums::RepoRemoteChange;
pub use self::enums::StaticDeltaGenerateOpt;

mod flags;
#[cfg(any(feature = "v2017_13", feature = "dox"))]
pub use self::flags::ChecksumFlags;
pub use self::flags::DiffFlags;
pub use self::flags::GpgSignatureFormatFlags;
pub use self::flags::RepoCommitModifierFlags;
#[cfg(any(feature = "v2015_7", feature = "dox"))]
pub use self::flags::RepoCommitState;
pub use self::flags::RepoCommitTraverseFlags;
pub use self::flags::RepoListObjectsFlags;
pub use self::flags::RepoListRefsExtFlags;
pub use self::flags::RepoPruneFlags;
pub use self::flags::RepoPullFlags;
pub use self::flags::RepoResolveRevExtFlags;
pub use self::flags::SePolicyRestoreconFlags;
pub use self::flags::SysrootSimpleWriteDeploymentFlags;
pub use self::flags::SysrootUpgraderFlags;
pub use self::flags::SysrootUpgraderPullFlags;

pub mod functions;

mod constants;
pub use self::constants::COMMIT_GVARIANT_STRING;
#[cfg(any(feature = "v2020_4", feature = "dox"))]
pub use self::constants::COMMIT_META_KEY_ARCHITECTURE;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub use self::constants::COMMIT_META_KEY_COLLECTION_BINDING;
#[cfg(any(feature = "v2017_7", feature = "dox"))]
pub use self::constants::COMMIT_META_KEY_ENDOFLIFE;
#[cfg(any(feature = "v2017_7", feature = "dox"))]
pub use self::constants::COMMIT_META_KEY_ENDOFLIFE_REBASE;
#[cfg(any(feature = "v2017_9", feature = "dox"))]
pub use self::constants::COMMIT_META_KEY_REF_BINDING;
#[cfg(any(feature = "v2017_13", feature = "dox"))]
pub use self::constants::COMMIT_META_KEY_SOURCE_TITLE;
#[cfg(any(feature = "v2014_9", feature = "dox"))]
pub use self::constants::COMMIT_META_KEY_VERSION;
pub use self::constants::DIRMETA_GVARIANT_STRING;
pub use self::constants::FILEMETA_GVARIANT_STRING;
#[cfg(any(feature = "v2021_1", feature = "dox"))]
pub use self::constants::METADATA_KEY_BOOTABLE;
#[cfg(any(feature = "v2021_1", feature = "dox"))]
pub use self::constants::METADATA_KEY_LINUX;
#[cfg(any(feature = "v2018_9", feature = "dox"))]
pub use self::constants::META_KEY_DEPLOY_COLLECTION_ID;
#[cfg(any(feature = "v2018_3", feature = "dox"))]
pub use self::constants::ORIGIN_TRANSIENT_GROUP;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
pub use self::constants::REPO_METADATA_REF;
#[cfg(any(feature = "v2020_4", feature = "dox"))]
pub use self::constants::SIGN_NAME_ED25519;
pub use self::constants::SUMMARY_GVARIANT_STRING;
pub use self::constants::SUMMARY_SIG_GVARIANT_STRING;
pub use self::constants::TREE_GVARIANT_STRING;

#[doc(hidden)]
pub mod traits {
    pub use super::AsyncProgressExt;
    pub use super::ContentWriterExt;
    pub use super::MutableTreeExt;
    pub use super::RepoFileExt;
    #[cfg(any(feature = "v2018_6", feature = "dox"))]
    pub use super::RepoFinderExt;
    #[cfg(any(feature = "v2018_6", feature = "dox"))]
    pub use super::RepoFinderAvahiExt;
    #[cfg(any(feature = "v2018_6", feature = "dox"))]
    pub use super::RepoFinderMountExt;
    #[cfg(any(feature = "v2018_6", feature = "dox"))]
    pub use super::RepoFinderOverrideExt;
    #[cfg(any(feature = "v2020_2", feature = "dox"))]
    pub use super::SignExt;
}
