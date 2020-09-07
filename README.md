# include_rgba

A Rust procedural macro for convenient including of images, textures as RGBA pixel array.

## Usage

```rust
// The type is [u8; X] where X is the number of pixels * 4
// each element represents a channel value,
// so 4 elements make up a pixel
let rgba_data = include_rgba!("path/to/the/image.png")
```
