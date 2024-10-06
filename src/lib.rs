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

//! A simple parser and reader for Thumbs.db files.
//!
//! This uses the crate `ole` for parsing Thumbs.db files.
//! Then, it extracts the thumbnails.
//!
//! ## Example
//!
//! ```
//!
//! use thumbsdb::ThumbsDb;
//!
//! // Reads a Thumbs.db file
//! let thumbsdb = ThumbsDb::from_path("assets/Thumbs.db").unwrap();
//!
//! // Iterates through all thumbnails inside the Thumbs.db
//! for thumbnail in thumbsdb.iterate() {
//!   let mut buf = std::vec::Vec::<u8>::new();
//!
//!   // Extracts the thumbnail and saves it into a file
//!   match thumbsdb.extract_thumbnail(thumbnail, &mut buf) {
//!     Ok(()) => {
//!       use std::io::Write;
//!       let mut output_file
//!         = std::fs::File::create(format!("assets/streams/{}",
//!             thumbnail.name())).unwrap();
//!       output_file.write_all(&buf);
//!     },
//!     Err(e) => eprintln!("error: {}", e)
//!   };
//! }
//!
//! ```
//!
//! ## Compatibility
//!
//! This crate works fine for rust 1.9 or greater.


extern crate ole;

mod thumbsdb;
mod error;
mod catalog;
mod util;
mod thumbnail;
mod iterator;
pub use crate::thumbsdb::ThumbsDb;
pub use crate::error::Error;
pub use crate::thumbnail::Thumbnail;
pub use crate::iterator::ThumbnailIterator;


#[cfg(test)]
mod tests {
    use super::ThumbsDb;
    use std;
    #[test]
    fn it_works() {
      let t = ThumbsDb::from_path("assets/Thumbs.db").unwrap();
      for thumbnail in t.iterate() {
        println!("thumbnail={}", thumbnail);
        let mut buf = std::vec::Vec::<u8>::new();
        match t.extract_thumbnail(thumbnail, &mut buf) {
          Ok(()) => {
            use std::io::Write;
            let mut f = std::fs::File::create(format!("assets/streams/{}",
                thumbnail.name())).unwrap();
            match f.write_all(&buf) {
              Ok(_) => println!("{} extracted.", thumbnail.name()),
              Err(e) => eprintln!("Error: {}", e)
            }
          },
          Err(e) => eprintln!("error: {}", e)
        }
      }
    }
}
