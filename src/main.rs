use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

fn main() {
    let f = File::open("cbindgen-0.24.3-x86_64-apple-darwin.tar.gz").unwrap();
    let tar = GzDecoder::new(f);
    Archive::new(tar).unpack(".").unwrap();
}
