# iter-python

> **Python generator expressions ([`iter!`]) and "list" comprehensions ([`vec_it!`])**

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)][Repository]
[![Latest version](https://img.shields.io/crates/v/iter-python.svg)][crates.io]
[![Documentation](https://docs.rs/iter-python/badge.svg)][Documentation]
[![License](https://img.shields.io/crates/l/iter-python.svg)](https://github.com/danielhenrymantilla/iter-python-rs#license)

## Usage

 1. Add the following line to your `Cargo.toml`, under `[dependencies]`:

    ```toml
    iter-python = "0.9.0"
    ```

 1. Bring [`iter!`] and [(enhanced) `vec!`][`vec_it!`] into scope in you Rust code with:

    ```rust
    use ::iter_python::{iter, vec_it as vec};
    ```

## Example

```rust
//! Run with `cargo run --example readme`

use ::iter_python::{
    iter,           // "generator expressions"
    vec_it as vec,  // "list comprehensions"
};

fn main ()
{
    use ::itertools::Itertools; // .join() method

    let s = iter!(
        format!("{}!", s)
        for s in " Hello World ".split_whitespace()
        if !s.is_empty()
    ).join("\n");
    dbg!(&s);
    assert_eq!(
        s,
        "Hello!\nWorld!",
    );

    let v = vec![
        x
        for &x_opt in &[None, Some(42), None]
        if let Some(x) = x_opt
    ];
    dbg!(&v);
    assert_eq!(
        v,
        vec![42],  // classic vec! macro is still there
    );
}
```

See [`iter!`] and [`vec_it!`] for more examples.

[`iter!`]: https://docs.rs/iter-python/0.9.0/iter_python/macro.iter.html
[`vec_it!`]: https://docs.rs/iter-python/0.9.0/iter_python/macro.vec_it.html

[Repository]: https://github.com/danielhenrymantilla/iter-python-rs
[Documentation]: https://docs.rs/iter-python
[crates.io]: https://crates.io/crates/iter-python
