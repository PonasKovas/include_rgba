# include_rgba

[![Crates.io](https://img.shields.io/crates/v/include_rgba)](https://crates.io/crates/include_rgba)

A Rust procedural macro for convenient including of images, textures as RGBA pixel array.

**WARNING! I've found that for some reason this macro results in REALLY long compile times, (going up to 10 min for just a few small images, even with caching). I wouldn't suggest using this crate in any case ever.**

## Usage

```rust
// The type is [u8; X] where X is the number of pixels * 4
// each element represents a channel value,
// so 4 elements make up a pixel
let rgba_data = include_rgba!("path/to/the/image.png")
```
