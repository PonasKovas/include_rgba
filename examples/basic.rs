#![feature(proc_macro_hygiene)]

extern crate include_rgba;

fn main() {
    let image: [u8; 16] = include_rgba::include_rgba!("examples/image.png");

    println!("{:?}", image);
}
