# Introduction

A beginner project to try to understand how file compression works, and potentially implement that in rust.

The initial plan is to implement this using a rust crate called `flate2`
while building some placeholders for an in house compression algorithm which might take
slightly longer.

I've linked some notes below, both for myself and anyone else who wants to understand this repo.

# Structure

- This program is broken down into a pretty standard rust runnable format.
- There is **one binary** and **two libraries**.
    - The binary or the runnable is called `runner`
    - The 2 crates/libraries are `compressor` and `uncompresor`.

## Runner

This is the main runner and the only binary for the program.

As of now this contains the main parsing logic for the input (which was implemented
using [clap](https://docs.rs/clap/latest/clap/)).

## Compressor

This is the crate that is designed to compress the file.
As of now, the encoding method and algorithm would be pulled from this crate
[flate2](https://docs.rs/flate2/latest/flate2/).

The compression algorithm is still under some discussion, between 2 main choices
that are offered by flate. This will probably be considered after a little bit more code.

But for the most bit, I'll test the same file or so on both and look at potential advantages or drawbacks.

As of now, a `GzEncoder` and `ZLibEncoder` were both tested on a `6mb.pdf`.
These are the results obtained:
```shell
Length before compression: 6504095
Length after compression (GzEncoder): 27
Length after compression (ZLibEncoder): 15
```

For context this is the code that was used to test them
```rust
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
```
The ZLibEncoder does better, and this perhaps makes a little more sense once we try to understand
how each of these encoders are implemented internally. Below is some trivia about the rough structure
of each of these encoders and some of their design features which make them stand out.

### zlib
Zlib is a free and OSS software mainly designed for compression and decompression of files.
It's lossless, with pretty good compression ranges. The whole idea revolves around using `Huffman Coding` first in order to
generate a low entropy tree and then runs it by LZ277 to match the text and maintain something like a sliding window.

The main downside of this, apart from the complexity of implementation is that it doesnt contain a checksum in order to 
maintain the integrity of data. But perhaps that's why it is significantly easier to implement ahead of `gz`.

This is also why the length after compression is slightly smaller (6mb is not much of a measure but its great for now)
### gz
Another popular lossless data compression method. Mainly used on standalone files and is used in linux and unix based systems.
So naturally, if the pdf were larger **gzip would outperform zlib in terms of compression ratio**. Ideally gzip compresses file
with the intention of sending them across the internet (either as packets in IP or dataframes in ethernet), but this is more suited for
network bandwidth optimization.

Either way, one thing that I've noticed in either case is that there are 2 data structure implementations that I would need
to do myself in order to perform compression.
1. Huffman Coding (Huffman Tree, Node , etc)
2. LZ277 (sliding window, Look Ahead Buffer, etc)

So that is what I'm going to do.







