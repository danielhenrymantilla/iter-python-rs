# iter-python

> **Python generator expressions ([`i!`]) and "list" comprehensions ([`v!`])**

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](
https://github.com/danielhenrymantilla/iter-python-rs)
[![Latest version](https://img.shields.io/crates/v/iter-python.svg)](
https://crates.io/crates/iter-python)
[![Documentation](https://docs.rs/iter-python/badge.svg)](
https://docs.rs/iter-python)
[![MSRV](https://img.shields.io/badge/MSRV-1.42.0-white)](
https://gist.github.com/danielhenrymantilla/8e5b721b3929084562f8f65668920c33)
[![License](https://img.shields.io/crates/l/iter-python.svg)](
https://github.com/danielhenrymantilla/iter-python-rs/blob/master/LICENSE-ZLIB)
[![CI](https://github.com/danielhenrymantilla/iter-python-rs/workflows/CI/badge.svg)](
https://github.com/danielhenrymantilla/iter-python-rs/actions)

## Usage

 1. Add the following line to your `Cargo.toml`, under `[dependencies]`:

    ```toml
    iter-python = "0.10.0"
    ```

 1. Bring [`i!`] and [(enhanced) `v!`ec][`v!`] into scope in you Rust code with:

    ```rust
    use ::iter_python::prelude::*;
    ```

## Example

```rust
use ::iter_python::prelude::{*,
    // Not necessary, but since `v` supersedes stdlib's `vec`'s API,
    // you can do this if you feel like it.
    v as vec,
};

fn main ()
{
    // `i!` macro for comprehension "lists" (iterators):
    let infinite_odds = || i!(2 * n + 1 for n in 0..);
    let sums_of_odds = i!(infinite_odds().take(n).sum() for n in 1..);
    assert!(sums_of_odds.take(100).all(is_square));
 /* The above may be suprising, but is an obvious mathematical property
    once we represent it as:

    1> 1   3   5   M-1
           |   |    |
    3> 1---2   4   M-#
               |    |
    5> 1---2---3   M-n
                    |
    M> 1---#---n---n+1  where M=2n+1
 */

    // `v!` macro: like `vec!`, but supporting `i!`'s input as well.
    let v = v![
        2 * x
        for &x_opt in &[None, Some(21), None]
        if let Some(x) = x_opt
    ];
    assert_eq!(
        dbg!(v),
        vec![42],  // `v!` does indeed feature classic `vec!` semantics.
    );


    // A more advanced example: generate the following string…
    const MATRIX: &str = "\
+-----+-----+-----+-----+-----+
| a11 | a12 | a13 | a14 | a15 |
| a21 | a22 | a23 | a24 | a25 |
| a31 | a32 | a33 | a34 | a35 |
| a41 | a42 | a43 | a44 | a45 |
| a51 | a52 | a53 | a54 | a55 |
+-----+-----+-----+-----+-----+";
    const N: usize = 6;

    // … using only one allocation!
    // This is achieved by combining lazy iterators (`i!`) with
    // "lazy strings" / lazy `Display`ables: `lazy_format!`.
    use ::iter_python::macros::lazy_format; // prelude provides it as `f!`
    let line = || lazy_format!(
        "+-{}-+",
        "-+-".join(i!("---" for _ in 1..N))
    );
    let top_line = line();
    let body = "\n".join(i!(
        lazy_format!(
            "| {} |",
            " | ".join(i!(f!("a{i}{j}") for j in 1..N)),
        )
        for i in 1..N
    ));
    let bottom_line = line();
    // Heap-allocation and iterator consumption occurs here:
    let matrix = format!("{top_line}\n{body}\n{bottom_line}");
    assert_eq!(matrix, MATRIX);
}

fn is_square (n: u32)
  -> bool
{
    n == ((n as f64).sqrt().trunc() as u32).pow(2)
}
```

See [`iter!`] and [`vec!`] for more examples.


## `no_std` support

This crates supports `#![no_std]`, by disabling the default `"std"` feature.

[`i!`]: https://docs.rs/iter-python/0.10.0/iter_python/macros/macro.iter.html
[`iter!`]: https://docs.rs/iter-python/0.10.0/iter_python/macros/macro.iter.html
[`v!`]: https://docs.rs/iter-python/0.10.0/iter_python/macros/macro.vec.html
[`vec!`]: https://docs.rs/iter-python/0.10.0/iter_python/macros/macro.vec.html

[Repository]: https://github.com/danielhenrymantilla/iter-python-rs
[Documentation]: https://docs.rs/iter-python
[crates.io]: https://crates.io/crates/iter-python
