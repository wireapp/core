//! # Wire Persistence Layer
//!
//! This crate implements a thin persistence layer to store and retrieve data.
//!
//! In the future it will also allow setting storage callbacks if the platform
//! does not use a file-based persistence mechanism.
//!
//! This crate should ensure that data that is written to the disk is encrypted.
//!

use std::path::Path;

pub enum Error {}

pub struct Storage {}
impl Storage {
    /// Read data from a path.
    /// Returns the object of type `T` or an error if reading fails.
    pub fn read<T>(_path: &Path) -> Result<T, Error> {
        todo!();
    }

    /// Write data of type `T` to the path.
    /// Returns an error if it fails.
    pub fn write<T>(_data: T, _path: &Path) -> Result<(), Error> {
        todo!();
    }

    // TODO: callbacks
    // TODO: we could also flip it and register paths in the beginning and then
    //       only call read and write.
}
