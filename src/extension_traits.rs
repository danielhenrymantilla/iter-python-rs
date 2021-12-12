pub use ::join_lazy_fmt::Join;

/// Provides a convenience `.vec()` and `.dict()` adapters to collect an `i!(…)`
/// iterator expression into a [`Vec`] and a
/// [`HashMap`][`::std::collections::HashMap`] respectively.
///
/// # Example
///
/// ```rust
/// use ::iter_python::prelude::*;
///
/// assert_eq!(
///     i!(x for x in 0 .. 5).vec(),
///     v![x for x in 0 .. 5],
/// );
///
/// assert_eq!(
///     i!((x, x * x) for x in 1 ..= 2).dict(),
///     {
///         let mut map = ::std::collections::HashMap::new();
///         map.insert(1, 1);
///         map.insert(2, 4);
///         map
///     },
/// );
/// ```
#[cfg(feature = "std")]
#[cfg_attr(feature = "better-docs",
    doc(cfg(feature = "std")),
)]
pub trait IteratorExt : Sized + Iterator {
    fn vec<T> (self: Self)
      -> crate::__::std::vec::Vec<T>
    where
        Self : Iterator<Item = T>,
    {
        self.collect()
    }

    fn dict<K, V> (self: Self)
      -> crate::__::std::collections::HashMap<K, V>
    where
        Self : Iterator<Item = (K, V)>,
        K : ::core::hash::Hash + Eq,
    {
        self.collect()
    }
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "better-docs",
    doc(cfg(feature = "std")),
)]
impl<T> IteratorExt for T where Self : Iterator {}
