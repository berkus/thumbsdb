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

const JPEG_HEADER: [u8; 2] = [0xFF, 0xD8];


/// A thumbnail inside a Thumbs.db file.
pub struct Thumbnail {

  /// Filename of the Thumbnail picture
  filename: std::string::String,

  /// ID
  id: u32,

  /// MAC time
  time: u64
}


impl Thumbnail {

  /// Returns the name of the picture which the thumbnail comes from.
  pub fn name(&self) -> &str {
    &self.filename
  }

  /// Returns the ID of the thumbnail.
  ///
  /// This id is relative to the thumbs.db file.
  pub fn id(&self) -> u32 {
    self.id
  }

  /// Returns the win32 MAC time of the thumbnail.
  pub fn time(&self) -> u64 {
    self.time
  }
}

impl std::fmt::Display for Thumbnail {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
  	write!(f, "Thumbnail #{}, name={}, mac={}", self.id(),
				self.name(), self.time())
	}
}




impl<'ole> super::thumbsdb::ThumbsDb<'ole> {

  pub(crate) fn extract_thumbnails(&mut self)
      -> Result<(), super::error::Error>{
    use super::util::FromSlice;
    let n = usize::from_slice(&self.content[4 .. 8]);
    self.thumbnails = std::vec::Vec::with_capacity(n);
    let mut last_offset = 0;
    for _ in 0 .. n {
      let os = 16 + last_offset;
      let size = usize::from_slice(&self.content[os .. os + 4]);
      let id = u32::from_slice(&self.content[os + 4 .. os + 8]);
      let time = u64::from_slice(&self.content[os + 8 .. os + 16]);
      let rm = (size as i32) - 16;
      if rm > 0 {
        let mut name = std::string::String::with_capacity((rm as usize) / 2 + 1);
        let array = &self.content[os + 16 .. os + 16 + (rm as usize)];
        let mut i = 0;
        while i < (rm as usize) && array[i] != 0 {
          name.push(array[i] as char);
          i += 2;
        }
        let thumbnail = Thumbnail {
          filename: name,
          id: id,
          time: time
        };
        last_offset += size;
        self.thumbnails.push(thumbnail);
      }
    }
    Ok(())
  }

  /// Extract the thumbnail from the Thumbs.db file.
  ///
  /// ## Examples
  ///
  /// ```
  /// use thumbsdb;
  ///
  /// let mut thumbs =
  ///     thumbsdb::ThumbsDb::from_path("assets/Thumbs.db").unwrap();
  /// let thumbnail = thumbs.iterate().next().unwrap();
  /// let mut buffer = std::vec::Vec::<u8>::new();
  /// thumbs.extract_thumbnail(thumbnail, &mut buffer);
  /// ```
  pub fn extract_thumbnail(&self, thumbnail: &Thumbnail,
                            buf: &mut std::vec::Vec<u8>)
      -> Result<(), super::error::Error> {
    let name = thumbnail.id().to_string().chars().rev().collect::<String>();
    if let Some(entry) = self.ole.iterate().skip_while(
        |e| e.name() != &name).next() {
      use std::io::Read;
      let mut slice =
      self.ole.get_entry_slice(entry).map_err(super::error::Error::OLEError)?;
      slice.read_to_end(buf).map_err(super::error::Error::IOError)?;
      if &buf[12 .. 14] == &JPEG_HEADER {
        // hmmm
        for _ in 0 .. 12 {
          buf.remove(0);
        }
        Ok(())
      } else {
        Err(super::error::Error::NotJPEGFile)
      }
    } else {
      Err(super::error::Error::NoEntryAssociated)
    }
  }
}
