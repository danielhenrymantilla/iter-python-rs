

/// Write the most pervasive iterator adapters
/// ([filter]ing and [map]ping) as [Python generator expressions].
///
/// # Examples
///
/// ### Squaring even numbers
///
/// ```rust,edition2018
/// # use ::iter_python::iter;
/// let mut all_evens_squared = iter!(
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
/// ```rust,edition2018
/// # use ::iter_python::iter;
/// let mut numbers_binary = iter!(format!("{:02b}", x) for x in 1 ..= 3);
///
/// assert_eq!(numbers_binary.next(), Some("01".into()));
/// assert_eq!(numbers_binary.next(), Some("10".into()));
/// assert_eq!(numbers_binary.next(), Some("11".into()));
/// assert_eq!(numbers_binary.next(), None);
/// ```
///
/// ### You may also `filter` with `if let`:
///
/// ```rust,edition2018
/// # use ::iter_python::iter;
/// let strings = ["42", "0", "zero", "27"];
///
/// let parsed_as_i32s = iter!(s.parse::<i32>() for &s in &strings);
///
/// let total: i32 = Iterator::sum(iter!(
///     x
///     for res in parsed_as_i32s
///     if let Ok(x) = res
/// ));
///
/// assert_eq!(total, 42 + 0 + 27);
/// ```
///
/// ```rust,edition2018
/// # use ::iter_python::iter;
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
/// let no_rotten_apple = iter!(
///     fruit
///     for fruit in BASKET
///     if let Banana | Peach = fruit
/// );
///
/// assert!({no_rotten_apple}.all(Fruit::is_fresh));
/// ```
///
/// [filter]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter
/// [map]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.map
/// [Python generator expressions]: https://www.python.org/dev/peps/pep-0289
/// [`if let`]: https://doc.rust-lang.org/book/if-let.html
#[macro_export]
macro_rules! iter {
    (
        $mapped_expr:expr ,
        for $var:pat in $iterable:expr ,
        if $cond:expr $(,)?
    ) => (
        $iterable
            .into_iter()
            .filter_map(|$var| {
                if $cond {
                    Some($mapped_expr)
                } else {
                    None
                }
            })
    );

    (
        $mapped_expr:expr ,
        for $var:pat in $iterable:expr ,
        if let $( $pat:pat )|+ = $value:expr $(,)?
    ) => (
        $iterable
            .into_iter()
            .filter_map(|$var| {
                if let $( $pat )|+ = $value {
                    Some($mapped_expr)
                } else {
                    None
                }
            })
    );

    (
        $mapped_expr:expr ,
        for $var:pat in $iterable:expr $(,)?
    ) => (
        $iterable
            .into_iter()
            .map(|$var| $mapped_expr)
    );

    (@parsing_mapped_expr
        $mapped_expression:tt
        for
        $($rest:tt)*
    ) => ($crate::iter!(@parsing_for
        $mapped_expression
        [ for ] // for_body
        $($rest)*
    ));

    (@parsing_mapped_expr
        [ $($mapped_expression_tts:tt)* ]
        $current_tt:tt
        $($rest:tt)*
    ) => ($crate::iter!(@parsing_mapped_expr
        [ $($mapped_expression_tts)* $current_tt ]
        $($rest)*
    ));

    (@parsing_for
        [ $($mapped_expression_tts:tt)* ]
        [ $($for_body_tts:tt)* ]
    ) => ($crate::iter!(
        $($mapped_expression_tts)* ,
        $($for_body_tts)*,
    ));

    (@parsing_for
        [ $($mapped_expression_tts:tt)* ]
        [ $($for_body_tts:tt)* ]
        if
        $($rest:tt)*
    ) => ($crate::iter!(
        $($mapped_expression_tts)* ,
        $($for_body_tts)* ,
        if $($rest)*
    ));

    (@parsing_for
        $mapped_expression:tt
        [ $($for_body_tts:tt)* ]
        $current_tt:tt
        $($rest:tt)*
    ) => ($crate::iter!(@parsing_for
        $mapped_expression
        [ $($for_body_tts)* $current_tt ]
        $($rest)*
    ));

    (
        $($tt:tt)*
    ) => ($crate::iter!(@parsing_mapped_expr
        [] // mapped_expression
        $($tt)*
    ));
}

/// [Python "list" comprehensions]: same as [`iter!`],
/// but [`collect`]ed into a [`Vec`] instead.
///
/// # `vec_it!` or `vec!`?
///
/// **[`vec_it!`] fallbacks to [`::std::vec!`] functionality**,
/// thus allowing maximum compatiblity!
///
/// ### Example
/// ```rust,edition2018
/// use ::iter_python::vec_it as vec;
///
/// let v1 = vec![i for i in 1 ..= 4];
/// let v2 = vec![1, 2, 3, 4];
/// assert_eq!(v1, v2);
/// ```
///
/// It has not been named `vec` to prevent lints against ambiguous blob imports.
///
/// [`iter!`]: `iter`
/// [`collect`]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.collect
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [Python "list" comprehensions]: https://www.python.org/dev/peps/pep-0202
/// [`vec_it!`]: `vec_it`
/// [`::std::vec!`]: https://doc.rust-lang.org/std/macro.vec.html
#[macro_export]
macro_rules! vec_it {
    (
        $expr:expr ,
        for $var:pat in $iterable:expr
        $(
            ,
            $($if_cond:tt)*
        )?
    ) => (
        $crate::iter!(
            $expr,
            for $var in $iterable
            $(
                ,
                $($if_cond)*
            )?
        ).collect::<::std::vec::Vec<_>>()
    );

    (@parsing_mapped_expr
        $fallback_to_vec:tt
    ) => (
        ::std::vec! $fallback_to_vec
    );


    (@parsing_mapped_expr
        $mapped_expression:tt
        for
        $($rest:tt)*
    ) => ($crate::vec_it!(@parsing_for
        $mapped_expression
        [ for ] // for_body
        $($rest)*
    ));

    (@parsing_mapped_expr
        [ $($mapped_expression_tts:tt)* ]
        $current_tt:tt
        $($rest:tt)*
    ) => ($crate::vec_it!(@parsing_mapped_expr
        [ $($mapped_expression_tts)* $current_tt ]
        $($rest)*
    ));

    (@parsing_for
        [ $($mapped_expression_tts:tt)* ]
        [ $($for_body_tts:tt)* ]
    ) => ($crate::vec_it!(
        $($mapped_expression_tts)* ,
        $($for_body_tts)* ,
    ));

    (@parsing_for
        [ $($mapped_expression_tts:tt)* ]
        [ $($for_body_tts:tt)* ]
        if
        $($rest:tt)*
    ) => ($crate::vec_it!(
        $($mapped_expression_tts)* ,
        $($for_body_tts)*,
        if $($rest)*
    ));

    (@parsing_for
        $mapped_expression:tt
        [ $($for_body_tts:tt)* ]
        $current_tt:tt
        $($rest:tt)*
    ) => ($crate::vec_it!(@parsing_for
        $mapped_expression
        [ $($for_body_tts)* $current_tt ]
        $($rest)*
    ));

    (
        $($tt:tt)*
    ) => ($crate::vec_it!(@parsing_mapped_expr
        [] // mapped_expression
        $($tt)*
    ));
}
