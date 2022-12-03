//! Invariant functors.

use crate::functor::Functor;
use crate::higher::Higher;

/// Invariant functor (also known as exponential functor).
pub trait Invariant<MapB>: Higher {
    /// Transform a `Self<A>` into a `Self<B>` by providing a transformation from `A` to `B`
    /// and one from `B` to `A`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x = Some("1".to_string());
    /// let actual = x.imap(|s| s.parse::<i32>().unwrap(), |i| i.to_string());
    /// assert_eq!(Some(1), actual);
    /// ```
    fn imap<F, G>(self, f: F, g: G) -> Self::Target<MapB>
    where
        F: FnMut(Self::Param) -> MapB,
        G: FnMut(MapB) -> Self::Param;
}

impl<MapB, T: Functor<MapB>> Invariant<MapB> for T {
    fn imap<F, G>(self, f: F, _g: G) -> Self::Target<MapB>
    where
        F: FnMut(Self::Param) -> MapB,
        G: FnMut(MapB) -> Self::Param,
    {
        self.map(f)
    }
}
