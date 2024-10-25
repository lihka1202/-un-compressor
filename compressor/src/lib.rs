use std::env::Args;
use std::io::BufReader;
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::write::GzEncoder;

pub fn encode(source: String, target: String) {
    let metadata = std::fs::metadata(String::from("input/") + &source).unwrap();
    println!("Length before compression: {:?}", metadata.len());
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(source.as_bytes()).unwrap();
    let compressed = encoder.finish().unwrap();
    println!("Length after compression (GzEncoder): {:?}", compressed.len());

    // Write the ZLibEncoder code
    let mut zlib_encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    zlib_encoder.write_all(source.as_bytes()).unwrap();
    let compressed_zlib_encoder = zlib_encoder.finish().unwrap();
    println!("Length after compression (ZLibEncoder): {:?}", compressed_zlib_encoder.len());
}