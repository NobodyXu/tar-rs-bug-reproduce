use async_compression::tokio::bufread::GzipDecoder;
use tokio::{fs::File, io::BufReader};
use tokio_tar::Archive;

#[tokio::main]
async fn main() {
    let f = File::open("cbindgen-0.24.3-x86_64-apple-darwin.tar.gz")
        .await
        .unwrap();
    let tar = GzipDecoder::new(BufReader::new(f));
    Archive::new(tar).unpack(".").await.unwrap();
}
