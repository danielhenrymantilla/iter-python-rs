#![cfg_attr(feature = "better-docs",
    cfg_attr(all(), doc = include_str!("../README.md")),
    feature(doc_cfg),
)]
#![cfg_attr(not(feature = "better-docs"),
    doc = "See [crates.io](https://crates.io/crates/iter-python)"
)]
#![cfg_attr(not(feature = "better-docs"),
    doc = "for more info about this crate."
)]

#![cfg_attr(not(doc),
    no_std,
)]

#![forbid(unsafe_code)]

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{
        all, any,
        macros::{
            lazy_format as f,
            iter as i,
        },
        extension_traits::{
            Join as _,
        },
    };

    #[doc(no_inline)]
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "better-docs",
        doc(cfg(feature = "std")),
    )]
    pub use crate::{
        extension_traits::IteratorExt as _,
        macros::vec as v,
    };
}

pub
mod extension_traits;

pub
mod macros;

/// Python's `all(iterable)` function.
///
/// # Example
///
/// ```rust,edition2018
/// use ::iter_python::*;
///
/// fn is_square (n: u32) -> bool
/// {
///     ((n as f64).sqrt().trunc() as u32).pow(2) == n
/// }
///
/// let odds = || iter!(2 * n + 1 for n in 0 ..);
///
/// let sums_of_odds = iter!(odds().take(n).sum() for n in 1 .. 20);
///
/// assert!(all(iter!(is_square(sum_of_odds) for sum_of_odds in sums_of_odds)));
/// ```
#[inline]
pub
fn all (
    iterable: impl IntoIterator<Item = bool>,
) -> bool
{
    iterable
        .into_iter()
        .all(::core::convert::identity)
}

/// Python's `any(iterable)` function.
///
/// # Example
///
/// ```rust,edition2018
/// use ::iter_python::*;
///
/// fn is_not_prime (n: usize) -> bool
/// {
///     any(iter!(n % k == 0 for k in (2 ..).take_while(|&k| k * k <= n)))
/// }
///
/// assert!(is_not_prime(91));
/// ```
#[inline]
pub
fn any (
    iterable: impl IntoIterator<Item = bool>,
) -> bool
{
    iterable
        .into_iter()
        .any(::core::convert::identity)
}

#[doc(hidden)] /** Not part of the public API */ pub
mod __ {
    pub use core;

    #[cfg(feature = "std")] pub
    extern crate std;
}
