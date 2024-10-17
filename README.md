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

But for the most bit, I'll test the same file or so on both and look at potential advantages or drawbacks





