#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod archive;
pub mod error;
pub mod file;
pub mod metadata;

#[cfg(feature = "std")]
pub use archive::Archive;
pub use archive::{Directory, DirectoryLocator};

#[cfg(feature = "std")]
use std::io::*;
/// Finds all the files in a single-disk archive.
///
/// This is equivalent to
///
/// ```
/// # use zip::*; fn f(disk: std::fs::File) -> std::io::Result<()> {
/// DirectoryLocator::from_io(disk)?.into_directory()?.seek_to_files::<metadata::std::Full>()
/// # ;Ok(())}
/// ```
#[cfg(feature = "std")]
pub fn files(disk: impl Read + Seek) -> Result<impl Iterator<Item = Result<file::File>>> {
    DirectoryLocator::from_io(disk)?
        .into_directory()?
        .seek_to_files()
}
