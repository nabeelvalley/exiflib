# Exif Lib

Library for Parsing EXIF data/metadata from image files

## Usage

### Terminal

From the terminal you can run `cargo run` with a path to a file:

```sh
cargo run './path/to/file.jpg
```

To print out some basic EXIF data. For now this will just give the Tag ID and value, the ID can then be looked up on an [EXIF Tag Name List](https://exiftool.org/TagNames/EXIF.html) to see what it corresponds to

The library should work with any file formats that store EXIF data

### Library

The Library exposed via the `exif` module will parse EXIF data from the provided file's bytes using:

```rs
let exif = exif::parse(file); // file is `&[u8]`
```


## References

Implementation references and guidance for image formats from:

- [`rawloader`](https://docs.rs/rawloader/latest/rawloader/index.html)
- [`libopenraw`](https://libopenraw.freedesktop.org/)
- [`image`](https://docs.rs/image/0.5.4/image/index.html)
- [EXIF Tool Tag Names](https://exiftool.org/TagNames/EXIF.html)
- [EXIF Viewer](http://exif-viewer.com/)
- [Fujifilm EXIF Viewer](https://greybeard.org.uk/exif3/)
- [COMPSCI 365/590F - Bit Twiddling File Formats, Parsing EXIF](https://people.cs.umass.edu/~liberato/courses/2018-spring-compsci365+590f/lecture-notes/05-bit-twiddling-file-formats-parsing-exif/)
- [Description of Exif file format (MIT Media)](https://www.media.mit.edu/pia/Research/deepview/exif.html)
- [Exif Explanation](http://gvsoft.no-ip.org/exif/exif-explanation.html#ExifIFDTags)
- [Exif Specification](http://web.archive.org/web/20131019050323/http://www.exif.org/specifications.html)