Rusty-git
=========

[![Build Status](https://travis-ci.org/keirlawson/rusty-git.svg?branch=master)](https://travis-ci.org/keirlawson/rusty-git)
[![Latest version](https://img.shields.io/crates/v/rustygit.svg)](https://crates.io/crates/rustygit)
[![Documentation](https://docs.rs/rustygit/badge.svg)](https://docs.rs/rustygit)

A high-level library for interacting with `git`

## Usage

Add the following to your `cargo.toml`:

```toml
[dependencies]
rustygit = "0.2"
```

```rust
use rustygit;

let repo = rustygit::Repository::new(".");
let branches = repo.list_branches().unwrap();

println!("branches:");
for branch in branches {
    println!("{}", branch);
}
```

## Comaprrison with [git2-rs](https://github.com/rust-lang/git2-rs)

Git2-rs is a mature and featureful Git library and unlike this library does not require that `git` be on the users $PATH.

This library does however have a few advantages over git2-rs:
* Pure Rust rather than bindings to a C++ library, making for easier cross-compilation.
* Works with git's SSH agent on Windows (libssh, which is used by git2-rs is unable to at present, making using SSH not possible on Windows)
* Provides a higher level API requiring less knowledge of Git internals
