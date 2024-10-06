# ThumbsDB

[![Crates.io](https://img.shields.io/crates/v/thumbsdb.svg)](https://crates.io/crates/thumbsdb)
[![Crates.io](https://img.shields.io/crates/d/thumbsdb.svg)](https://crates.io/crates/thumbsdb)
[![license](http://img.shields.io/badge/license-WTFPL-blue.svg)](https://github.com/zadlg/thumbsdb/blob/master/LICENSE)

A simple parser and reader for Microsoft Thumbs.db files.

This includes a basic parser, which validates the format of the given file, and
a reader for extracting thumbnails.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
thumbsdb = "0.1"
```

## Example

```rust
use thumbsdb;
use std::io::Write;

let mut file = std::fs::File::open("assets/Thumbs.db").unwrap();

// We're going to extract the thumbnails from the Thumbs.db
let thumbs = thumbsdb::new(file).unwrap();
for thumbnail in thumbs.iterate() {
  let mut buffer = std::vec::Vec::<u8>::new();
  thumbs.extract_thumbnail(thumbsnail, &mut buffer);

  // Saves the extracted thumbnail
  let mut extracted_file = std::fs::File::create(format!("assets/streams/{}",
    thumbnail.name())).unwrap();
  extracted_file.write_all(&buffer[..]);
}
```

## Releases

Release notes are available in [RELEASES.md](RELEASES.md).

## MSRV

cargo-msrv says `thumbsdb` works with rust 1.61 and greater.

## License

<http://www.wtfpl.net/about/>
