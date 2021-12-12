pub use ::join_lazy_fmt::lazy_format;

/// (shortname: `i!`) — Write the most pervasive iterator adapters
/// ([filter]ing and [map]ping) as [Python generator expressions].
///
/// # Examples
///
/// ### Squaring even numbers
///
/// ```rust
/// use ::iter_python::prelude::*;
///
/// let mut all_evens_squared = i!(
///     x * x
///     for x in (0 ..)
///     if x % 2 == 0
/// );
/// assert_eq!(all_evens_squared.next(), Some(0));
/// assert_eq!(all_evens_squared.next(), Some(4));
/// assert_eq!(all_evens_squared.next(), Some(16));
/// ```
///
/// ### `filter`ing is optional, such as in Python:
///
/// ```rust
/// use ::iter_python::prelude::*;
///
/// let mut numbers_binary = i!(format!("{:02b}", x) for x in 1 ..= 3);
///
/// assert_eq!(numbers_binary.next(), Some("01".into()));
/// assert_eq!(numbers_binary.next(), Some("10".into()));
/// assert_eq!(numbers_binary.next(), Some("11".into()));
/// assert_eq!(numbers_binary.next(), None);
/// ```
///
/// ### You may also `filter` with `if let`:
///
/// ```rust
/// use ::iter_python::prelude::*;
///
/// let strings = ["42", "0", "zero", "27"];
///
/// let parsed_as_i32s = i!(s.parse::<i32>() for &s in &strings);
///
/// let total: i32 = Iterator::sum(i!(
///     x
///     for res in parsed_as_i32s
///     if let Ok(x) = res
/// ));
///
/// assert_eq!(total, 42 + 0 + 27);
/// ```
///
/// ```rust
/// use ::iter_python::prelude::*;
///
/// enum Fruit { Banana, Peach, RottenApple }
/// use Fruit::*;
///
/// impl Fruit {
///     fn is_fresh (&self) -> bool
///     {
///         if let RottenApple = self {
///             false
///         } else {
///             true
///         }
///     }
/// }
///
/// static BASKET: &[Fruit] = &[Banana, RottenApple, Peach, Banana];
///
/// let no_rotten_apple = i!(
///     fruit
///     for fruit in BASKET
///     if let Banana | Peach = fruit
/// );
///
/// assert!({no_rotten_apple}.all(Fruit::is_fresh));
/// ```
///
/// ### You can also nest / combine iterators
///
/// ```rust
/// use ::iter_python::prelude::*;
///
/// assert_eq!(
///     i!((i, j) for i in 0 .. 3 for j in 0 .. 2).vec(),
///     vec![
///         (0, 0),
///         (0, 1),
///         (1, 0),
///         (1, 1),
///         (2, 0),
///         (2, 1),
///     ],
/// );
/// ```
///
/// #### With the same `if` guards as with the single case:
///
/// ```rust
/// use ::iter_python::prelude::*;
///
/// assert_eq!(
///     i!(
///         (i, j)
///         for i in 0 .. 4
///         if i != 2
///         for j in 0 .. i
///         if j != 1
///     ).vec(),
///     vec![
///         (1, 0),
///         (3, 0),
///         (3, 2),
///     ]
/// )
/// ```
/// [filter]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter
/// [map]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.map
/// [Python generator expressions]: https://www.python.org/dev/peps/pep-0289
/// [`if let`]: https://doc.rust-lang.org/book/if-let.html
#[macro_export]
macro_rules! iter {
    (
        $($input:tt)*
    ) => ($crate::__munch_iter_args! {
        munching[
            $($input)*
        ]
        current_elem[]
        to_endpoint[]
    });
}
#[doc(inline)]
pub use iter;

/// Split the input using `for` and `if`.
#[doc(hidden)] /** Not part of the public API */ #[macro_export]
macro_rules! __munch_iter_args {
    // New `for` *nested* (no `if` in between)
    (
        munching[
            for $($rest:tt)*
        ]
        current_elem[
            for $($acc_for:tt)*
        ]
        to_endpoint[
            $($out:tt)*
        ]
    ) => ($crate::__munch_iter_args! {
        munching[
            $($rest)*
        ]
        current_elem[ for ]
        to_endpoint[
            $($out)*
            [for $($acc_for)*]
            [] // encountered `for` after `for` => no `if` guard
        ]
    });

    // New `for`
    (
        munching[
            for $($rest:tt)*
        ]
        current_elem $current_elem:tt
        to_endpoint[
            $($out:tt)*
        ]
    ) => ($crate::__munch_iter_args! {
        munching[
            $($rest)*
        ]
        current_elem[ for ]
        to_endpoint[
            $($out)*
            $current_elem
        ]
    });

    // New `if`
    (
        munching[
            if $($rest:tt)*
        ]
        current_elem $current_elem:tt
        to_endpoint[
            $($out:tt)*
        ]
    ) => ($crate::__munch_iter_args! {
        munching[
            $($rest)*
        ]
        current_elem[ if ]
        to_endpoint[
            $($out)*
            $current_elem
        ]
    });

    // Neither `for` nor `if`: just accumulate that `:tt`
    (
        munching[
            $current_tt:tt $($rest:tt)*
        ]
        current_elem[
            $($current_elem:tt)*
        ]
        to_endpoint $to_endpoint:tt
    ) => ($crate::__munch_iter_args! {
        munching[
            $($rest)*
        ]
        current_elem[
            $($current_elem)* $current_tt
        ]
        to_endpoint $to_endpoint
    });

    // END: nothing left to munch
    (
        munching[]
        current_elem $current_elem:tt
        to_endpoint[
            $($out:tt)*
        ]
    ) => ($crate::__endpoint! {
        $($out)*
        $current_elem
    })
}

#[doc(hidden)] /** Not part of the public API */ #[macro_export]
macro_rules! __endpoint {
    // Non-last `for`-&-`if`
    (
        $mapped_expr:tt
        [for $var:pat in $iterable:expr]
        [if $($filter:tt)*]
        $($rest:tt)+
    ) => (
        $crate::__::core::iter::Iterator::flat_map(
            $crate::__::core::iter::IntoIterator::into_iter(
                $iterable
            ),
            move |$var| {
                $crate::__::core::iter::Iterator::flatten(
                    $crate::__::core::iter::IntoIterator::into_iter(
                        if $($filter)* {
                            $crate::__::core::option::Option::Some(
                                $crate::__endpoint!(
                                    $mapped_expr
                                    $($rest)*
                                )
                            )
                        } else {
                            $crate::__::core::option::Option::None
                        }
                    )
                )
            },
        )
    );

    // Non-last `for` (no `if`!)
    (
        $mapped_expr:tt
        [for $var:pat in $iterable:expr]
        [/* no filter */]
        $($rest:tt)+
    ) => (
        $crate::__::core::iter::Iterator::flat_map(
            $crate::__::core::iter::IntoIterator::into_iter(
                $iterable
            ),
            move |$var| $crate::__endpoint!($mapped_expr $($rest)*),
        )
    );

    // Last `for`-&-`if`
    (
        [$mapped_expr:expr]
        [for $var:pat in $iterable:expr]
        [if $($filter:tt)*]
        /* no rest */
    ) => (
        $crate::__::core::iter::Iterator::filter_map(
            $crate::__::core::iter::IntoIterator::into_iter(
                $iterable
            ),
            move |$var| if $($filter)* {
                $crate::__::core::option::Option::Some($mapped_expr)
            } else {
                $crate::__::core::option::Option::None
            },
        )
    );

    // Last `for` (no `if`!)
    (
        [$mapped_expr:expr]
        [for $var:pat in $iterable:expr]
        $([/* no filter */])?
        /* no rest */
    ) => (
        $crate::__::core::iter::Iterator::map(
            $crate::__::core::iter::IntoIterator::into_iter(
                $iterable
            ),
            move |$var| $mapped_expr,
        )
    );
}

/// (shortname: `v!`) — [Python "list" comprehensions]: same as [`i!`],
/// but [`collect`]ed into a [`Vec`] instead.
///
/// # `v!` or `vec!`?
///
/// **[`v!`] fallbacks to [`::std::vec!`] functionality**,
/// thus allowing maximum compatiblity!
///
/// ### Example
/// ```rust
/// use ::iter_python::macros::vec;
///
/// let v1 = vec![i for i in 1 ..= 4];
/// let v2 = vec![1, 2, 3, 4];
/// assert_eq!(v1, v2);
/// ```
///
/// It has not been named `vec` to prevent lints against ambiguous blob imports.
///
/// [`i!`]: `iter`
/// [`collect`]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.collect
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [Python "list" comprehensions]: https://www.python.org/dev/peps/pep-0202
/// [`v!`]: `vec`
/// [`::std::vec!`]: https://doc.rust-lang.org/std/macro.vec.html
#[cfg(feature = "std")]
#[cfg_attr(feature = "better-docs",
    doc(cfg(feature = "std")),
)]
#[macro_export]
macro_rules! v {
    // std syntax compat.
    (
        $($e:expr),* $(,)?
    ) => (
        $crate::__::std::vec! { $($e),* }
    );

    // std syntax compat.
    (
        $e:expr ; $count:expr
    ) => (
        $crate::__::std::vec! { $e; $count }
    );

    (
        $($otherwise:tt)*
    ) => (
        <
            $crate::__::std::vec::Vec<_>
            as
            $crate::__::core::iter::FromIterator<_>
        >::from_iter(
            $crate::macros::iter!( $($otherwise)* )
        )
    );
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "better-docs",
    doc(cfg(feature = "std")),
)]
#[doc(inline)]
pub use crate::v as vec;
