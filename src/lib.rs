#![cfg_attr(feature = "nightly",
    feature(external_doc)
)]
#![cfg_attr(feature = "nightly",
    doc(include = "../README.md")
)]
#![cfg_attr(not(feature = "nightly"),
    doc = "See [crates.io](https://crates.io/crates/iter-python)"
)]
#![cfg_attr(not(feature = "nightly"),
    doc = "for more info about this crate."
)]

#[doc(no_inline)]
pub use ::join_lazy_fmt::Join;

#[doc(inline)]
pub use ::join_lazy_fmt::lazy_format as f;

/// Reexport a trait that [**should be in the prelude**](
/// https://internals.rust-lang.org/t/pre-rfc-add-fromiterator-to-the-prelude/4324)
#[doc(hidden)]
pub
use core::iter::FromIterator;

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
        .all(core::convert::identity)
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
        .any(core::convert::identity)
}
