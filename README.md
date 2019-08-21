# mandelbrot-rust

a fractal generator written in rust using the [image](https://crates.io/crates/image) library.

## usage

`./mandelbrot-rust [n_iterations = 100] [width = 5000] [height = 5000]`

## how to compile

- Install the [rust toolchain](https://www.rust-lang.org/tools/install) and run `cargo buid --release`.
- The binary can be found in the `target/release/` folder, you can also run the program directly with `cargo run --release`
