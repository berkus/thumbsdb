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

use ole;
use thiserror::Error;

/// Errors related to the process of parsing and reading.
#[derive(Debug, Error)]
pub enum Error {
    /// This happens if an error occured with OLE.
    #[error("OLE error {0}")]
    OLEError(#[from] ole::Error),

    /// This happens if the given file isn't a Thumbs.db file.
    #[error("This file doesn't seem to be a Thumbs.db")]
    NotThumbsDbFile,

    /// Classic std::io::Error.
    #[error("IO error {0}")]
    IOError(std::io::Error),

    /// This happens if a thumbnail isn't a JPEG file.
    #[error("This thumbnail is not a JPEG file")]
    NotJPEGFile,

    /// This happens if a thumbnail is declared inside the Catalog entry, but
    /// The entry doesn't actually exist inside the Thumbs.db.
    #[error("Unable to find an entry for this thumbnail")]
    NoEntryAssociated,
}
