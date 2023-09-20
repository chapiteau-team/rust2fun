//! ApN.

use rust2fun_macros::ap_n;

use crate::and_then::AndThen;
use crate::functor::Functor;
use crate::higher::Higher;
use crate::semigroupal::Semigroupal;

/// Trait holding the `apXX` series of methods.
pub trait ApN<Z>: AndThen<Z> {
    /// Is a binary version of [Apply::ap].
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let f = Some(|x, y| x + y);
    /// assert_eq!(Some(3), f.ap2(Some(1), Some(2)));
    /// ```
    ///
    /// [Apply::ap]: crate::apply::Apply::ap
    #[inline]
    fn ap2<A, B>(self, fa: Self::Target<A>, fb: Self::Target<B>) -> Self::Target<Z>
    where
        Self::Param: FnMut(A, B) -> Z,
        Self::Target<A>: Semigroupal<B, Target<B> = Self::Target<B>>
            + Higher<Target<(A, B)> = Self::Target<(A, B)>>,
        Self::Target<(A, B)>: Functor<Z, Target<Z> = Self::Target<Z>> + Clone,
        Self: Sized,
    {
        let product = fa.product(fb);
        self.and_then(|mut f| product.clone().map(move |(a, b)| f(a, b)))
    }

    /// Is a ternary version of [Apply::ap].
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let f = Some(|x, y, z| x + y + z);
    /// assert_eq!(Some(6), f.ap3(Some(1), Some(2), Some(3)));
    /// ```
    ///
    /// [Apply::ap]: crate::apply::Apply::ap
    #[inline]
    fn ap3<A, B, C>(
        self,
        fa: Self::Target<A>,
        fb: Self::Target<B>,
        fc: Self::Target<C>,
    ) -> Self::Target<Z>
    where
        Self::Param: FnMut(A, B, C) -> Z,
        Self::Target<A>: Semigroupal<B, Target<B> = Self::Target<B>>
            + Higher<Target<(A, B)> = Self::Target<(A, B)>>,
        Self::Target<(A, B)>: Semigroupal<C, Target<C> = Self::Target<C>>
            + Higher<Target<((A, B), C)> = Self::Target<((A, B), C)>>,
        Self::Target<((A, B), C)>: Functor<Z, Target<Z> = Self::Target<Z>> + Clone,
        Self: Sized,
    {
        let product = fa.product(fb).product(fc);
        self.and_then(|mut f| product.clone().map(move |((a, b), c)| f(a, b, c)))
    }

    ap_n!(4);
    ap_n!(5);
    ap_n!(6);
    ap_n!(7);
    ap_n!(8);
    ap_n!(9);
    ap_n!(10);
    ap_n!(11);
    ap_n!(12);
}

impl<Z, T: AndThen<Z>> ApN<Z> for T {}
