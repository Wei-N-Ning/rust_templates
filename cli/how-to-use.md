# Rust CLI project template

Inspired by the 2022 book, _System Programming with Rust_

## How to use this project template

Delete the `target` directory then copy the contents to your project dir.

In CLion, open the **TODO** tab (or Menu | View | Tool Windows | TODO), and fix all the TODO
marks (replace the name etc. with yours).

---
hint: If you don't use CLion, simply search for `TODO:` and fix all the occasions.

---

Run `cargo test` and ensure all the tests pass (unit tests + integration tests).

(optional) Run `cargo +nightly bench` and ensure all the benchmarking tests pass.

Create a `.gitignore` file and paste the following lines:

```gitignore
target
Cargo.lock
**/*.rs.bk
```

Create a `README.md` file and remove the `how-to-use.md` file.

Remove all the unused files by your project.
