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

/// A simple iterator for thumbnails inside a Thumbs.db file.
pub struct ThumbnailIterator<'ole> {
  thumbsdb: &'ole super::thumbsdb::ThumbsDb<'ole>,
  curr: usize
}


impl<'ole> ThumbnailIterator<'ole> {

  /// Constructs an iterator for thumbnails, from a thumbsdb object.
  pub fn new(thumbsdb: &'ole super::thumbsdb::ThumbsDb) ->
  ThumbnailIterator<'ole> {
    ThumbnailIterator {
      thumbsdb: thumbsdb,
      curr: 0
    }
  }
}

impl<'ole> Iterator for ThumbnailIterator<'ole> {
  type Item = &'ole super::thumbnail::Thumbnail;

  fn next(&mut self) -> Option<&'ole super::thumbnail::Thumbnail> {
    let thumbnails = &self.thumbsdb.thumbnails;
    if self.curr == thumbnails.len() {
      None
    } else {
      let r = Some(&thumbnails[self.curr]);
      self.curr += 1;

      r
    }
  }
}
