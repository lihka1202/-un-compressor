mod huffman;

use std::env::Args;
use std::io::BufReader;
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::write::GzEncoder;

pub fn encode(source: String, target: String) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    let metadata = std::fs::metadata(&source).unwrap();
    println!("Length before compression: {:?}", metadata.len());
    // Add original length
    res.push(metadata.len());
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(source.as_bytes()).unwrap();
    let compressed = encoder.finish().unwrap();
    println!("Length after compression (GzEncoder): {:?}", compressed.len());
    res.push(compressed.len() as u64);

    // Write the ZLibEncoder code
    let mut zlib_encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    zlib_encoder.write_all(source.as_bytes()).unwrap();
    let compressed_zlib_encoder = zlib_encoder.finish().unwrap();
    println!("Length after compression (ZLibEncoder): {:?}", compressed_zlib_encoder.len());
    res.push(compressed_zlib_encoder.len() as u64);

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode() {
        let source = String::from("../input/6mb.pdf");
        let target = String::from("input/6mb.pdf");
        let res = encode(source, target);
        assert_eq!(res.len(), 3);
    }
}