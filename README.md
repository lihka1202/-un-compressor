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
- 