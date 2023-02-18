# Cargo basics and the Rust ecosystem

Cargo projects are called *crates*, which encompasses both libraries and
full-on binary-generating programs.

## New crate

Creating a new crate in Rust is as simple as doing `cargo new <name>` to
create it in a new directory or `cargo init` to setup the current directory as
a Cargo project.

You can specify the `--vcs=none` option if you want to stop Cargo from
generating a new git repo for the crate.

## Rust ecosystem

### [crates.io](https://crates.io)

[crates.io](https://crates.io) is the Rust package repository, where anyone can
publish their crate, most often as a library.

The website also allows for the publishing of full Rust programs using the
`cargo install` command.
Notable Rust applications installable in this way are the modern rewrites of
Linux utilities like `fd` for `find`, `bat` for `cat`, etc...

### Documentation

Rust documentation is standardised. Instead of having external tools like
Javadoc in Java or Sphinx in Python, simply running `cargo doc --no-deps` will
generate the documentation for your current crate.
This standard allows much easier navigation through all documentation, and
provides many useful utilities for both crate users and maintainers.

Perusing through documentation might be a little daunting right now since you
do not have the keys to understand everything. You'll be better armed to read
Rust documentation around the time when exercises start using external
libraries.

### Using external crates

As you'll notice later when creating your first project, cargo uses a
`Cargo.toml` file as a configuration file for the current project.
Far from the arcane horrors of files with infamous names like `Makefile`,
`CMakeLists.txt` or `meson.build`, Rust package management is extremely simple.

Inside the `Cargo.toml`, the `[dependencies]` field is used to list, well,
dependencies. Most dependencies you'll use will be specified in a simple
key/value format like `csv = "1.2.0"`.

A slightly more complex syntax has to be used for more advanced usage.
For example, say you wanted to use the `derive` feature-flag (basically a
compilation flag that enables or disables the compilation of certain parts of a
crate's code) of the `serde` crate, you'd write:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

This work can be simplified even further by using the `cargo add` command,
to which you can even provide features to add. The command for the previous
case would look like:
```
cargo add serde -F derive
```

With the vast, *vast* majority of crates, that's it; your code will just
compile.
