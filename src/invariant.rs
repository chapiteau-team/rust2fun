//! Invariant functors.

use crate::higher::Higher;

/// Invariant functor (also known as exponential functor).
pub trait Invariant: Higher {
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
    fn imap<B, F, G>(self, f: F, g: G) -> Self::Target<B>
        where F: FnMut(Self::Param) -> B,
              G: FnMut(B) -> Self::Param;
}

impl<A> Invariant for Option<A> {
    #[inline]
    fn imap<B, F, G>(self, f: F, _g: G) -> Option<B>
        where F: FnMut(Self::Param) -> B,
              G: FnMut(B) -> Self::Param {
        self.map(f)
    }
}

if_std! {
    use std::vec::Vec;

    impl<A> Invariant for Vec<A> {
        #[inline]
        fn imap<B, F, G>(self, f: F, _g: G) -> Self::Target<B>
            where F: FnMut(Self::Param) -> B,
                  G: FnMut(B) -> Self::Param {
            self.into_iter().map(f).collect::<Vec<_>>()
        }
    }
}