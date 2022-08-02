// TODO: replace this file with the real examples

use lib_cli::prelude::*;

// reference:
// https://www.reddit.com/r/rust/comments/5jax8b/how_does_the_examples_directory_work_with_cargo/

// run the examples with cargo like so
// cargo run --example add_example
//                     ^^^^^^^^^^^ filename add_example.rs

fn main() {
    println!("called library function add: {}", add(1, 2));
}
