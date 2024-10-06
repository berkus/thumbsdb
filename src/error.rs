//             DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyright (C) 2018 Thomas Bailleux <thomas@bailleux.me>
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.
//
// Author: zadig <thomas chr(0x40) bailleux.me>

use std;
use ole;

/// Errors related to the process of parsing and reading.
#[derive(Debug)]
pub enum Error {

  /// This happens if an error occured with OLE.
  OLEError(ole::Error),

  /// This happens if the given file isn't a Thumbs.db file.
  NotThumbsDbFile,

  /// Classic std::io::Error.
  IOError(std::io::Error),

  /// This happens if a thumbnail isn't a JPEG file.
  NotJPEGFile,

  /// This happens if a thumbnail is declared inside the Catalog entry, but
  /// The entry doesn't actually exist inside the Thumbs.db.
  NoEntryAssociated
}

impl std::error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::OLEError(ref e) => e.description(),
      Error::NotThumbsDbFile => "This file doesn't seem to be a Thumbs.db",
      Error::IOError(ref e) => e.description(),
      Error::NotJPEGFile => "This thumbnail is not a JPEG file",
      Error::NoEntryAssociated => "Unable to find an entry for this thumbnail"
    }
  }

  fn cause(&self) -> Option<&dyn std::error::Error> {
    match *self {
      Error::OLEError(ref e) => Some(e),
      Error::NotThumbsDbFile => None,
      Error::IOError(ref e) => Some(e),
      Error::NotJPEGFile => None,
      Error::NoEntryAssociated => None
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    use std::error::Error;
    write!(f, "{}", self.description())
  }
}
