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


/// A Thumbs.db file reader.
///
/// The crate `ole` is used for parsing the file (which is an OLE file).
///
/// # Basic Example
///
/// ```
/// use thumbsdb::ThumbsDb;
///
///
/// let mut file = std::fs::File::open("assets/Thumbs.db").unwrap();
/// let thumbsdb = ThumbsDb::new(file).unwrap();
///
/// for thumbnail in thumbsdb.iterate() {
///   println!("Thumbnail #{}, name={}", thumbnail.id(), thumbnail.name());
/// }
///
/// ```
pub struct ThumbsDb<'ole> {

  /// OLE object for parsing the file.
  pub(crate) ole: ole::Reader<'ole>,

  /// Content of the catalog.
  pub(crate) content: std::vec::Vec<u8>,

  /// Vector which stores all the thumbnails.
  pub(crate) thumbnails: std::vec::Vec<super::thumbnail::Thumbnail>
}

impl<'ole> ThumbsDb<'ole> {

  /// Constructs a new `ThumbsDb`.
  ///
  /// # Examples
  ///
  /// ```
  /// use thumbsdb;
  /// let f = std::fs::File::open("assets/Thumbs.db").unwrap();
  /// let mut hidden_thumbsdb = thumbsdb::ThumbsDb::new(f).unwrap();
  /// ```
  pub fn new<T: 'ole>(readable: T)
      -> Result<ThumbsDb<'ole>, super::error::Error>
    where T: std::io::Read {
    let mut thumbsdb = ThumbsDb {
      ole: ole::Reader::new(readable).map_err(super::error::Error::OLEError)?,
      content: std::vec::Vec::new(),
      thumbnails: std::vec::Vec::new()
    };

    thumbsdb.read_catalog()?;
    thumbsdb.extract_thumbnails()?;
    Ok(thumbsdb)
  }

  /// Constructs a new `ThumbsDb` from filepath.
  ///
  /// # Examples
  ///
  /// ```
  /// use thumbsdb;
  /// let mut hidden_thumbsdb =
  ///     thumbsdb::ThumbsDb::from_path("assets/Thumbs.db").unwrap();
  /// ```
  pub fn from_path(path: &str) -> Result<ThumbsDb, Box<dyn std::error::Error>> {
    match std::fs::File::open(path) {
      Ok(f) => match ThumbsDb::new(f) {
        Ok(t) => Ok(t),
        Err(e) => Err(Box::new(e))
      },
      Err(e) => Err(Box::new(e))
    }
  }

  /// Returns an iterator for thumbnails inside the Thumbs.db
  ///
  /// # Examples
  ///
  /// ```
  /// use thumbsdb;
  /// let mut t = thumbsdb::ThumbsDb::from_path("assets/Thumbs.db").unwrap();
  /// for thumbnail in t.iterate() {
  ///   println!("Entry #{}, filename={}", thumbnail.id(), thumbnail.name());
  /// }
  /// ```
  pub fn iterate(&self) -> super::iterator::ThumbnailIterator {
    super::iterator::ThumbnailIterator::new(self)
  }
}
