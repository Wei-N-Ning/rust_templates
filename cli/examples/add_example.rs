use lib_cli::mainlib;

// reference:
// https://www.reddit.com/r/rust/comments/5jax8b/how_does_the_examples_directory_work_with_cargo/

// run the examples with cargo like so
// cargo run --example add_example
//                     ^^^^^^^^^^^ filename add_example.rs

fn main() {
    println!("called mainlib function add: {}", mainlib::add(1, 2));
}
