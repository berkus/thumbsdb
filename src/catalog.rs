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

impl<'ole> super::thumbsdb::ThumbsDb<'ole> {

  pub(crate) fn read_catalog(&mut self) -> Result<(), super::error::Error> {
    if let Some(catalog_entry) = self.ole.iterate().skip_while(
         |e| e.name() != "Catalog").next() {
      use std::io::Read;
      self.content = std::vec::Vec::with_capacity(catalog_entry.len());
      let mut slice = self.ole.get_entry_slice(catalog_entry)
        .map_err(super::error::Error::OLEError)?;
      slice.read_to_end(&mut self.content)
          .map_err(super::error::Error::IOError)?;
      Ok(())
    } else {
      Err(super::error::Error::NotThumbsDbFile)
    }
  }
}
