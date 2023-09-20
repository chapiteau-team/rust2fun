//! MapN.

use rust2fun_macros::map_n;

use crate::functor::Functor;
use crate::higher::Higher;
use crate::semigroupal::Semigroupal;

/// Trait holding the `mapXX` series of methods.
pub trait MapN<B>: Semigroupal<B> {
    /// Combine two effectful values into a single effectful value using a binary function.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x = Some(1);
    /// let y = Some(2);
    /// let actual = x.map2(y, |x, y| x + y);
    /// assert_eq!(Some(3), actual);
    /// ```
    #[inline]
    fn map2<Z, F>(self, fb: Self::Target<B>, mut f: F) -> Self::Target<Z>
    where
        F: FnMut(Self::Param, B) -> Z,
        Self::Target<(Self::Param, B)>: Functor<Z, Target<Z> = Self::Target<Z>>,
        Self: Sized,
    {
        self.product(fb).map(|(a, b)| f(a, b))
    }

    /// Combine three effectful values into a single effectful value using a ternary function.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x = Some(1);
    /// let y = Some(2);
    /// let z = Some(3);
    /// let actual = x.map3(y, z, |x, y, z| x + y + z);
    /// assert_eq!(Some(6), actual);
    /// ```
    #[inline]
    fn map3<C, Z, F>(self, fb: Self::Target<B>, fc: Self::Target<C>, mut f: F) -> Self::Target<Z>
    where
        F: FnMut(Self::Param, B, C) -> Z,
        Self::Target<(Self::Param, B)>: Semigroupal<C, Target<C> = Self::Target<C>>
            + Higher<Target<((Self::Param, B), C)> = Self::Target<((Self::Param, B), C)>>,
        Self::Target<((Self::Param, B), C)>: Functor<Z, Target<Z> = Self::Target<Z>>,
        Self: Sized,
    {
        self.product(fb).product(fc).map(|((a, b), c)| f(a, b, c))
    }

    map_n!(4);
    map_n!(5);
    map_n!(6);
    map_n!(7);
    map_n!(8);
    map_n!(9);
    map_n!(10);
    map_n!(11);
    map_n!(12);

    /// Compose two effectful values discarding the result of the first.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x = Some(1);
    /// let y = Some(2);
    /// let actual = x.product_r(y);
    /// assert_eq!(Some(2), actual);
    /// ```
    #[inline]
    fn product_r(self, fb: Self::Target<B>) -> Self::Target<B>
    where
        Self::Target<(Self::Param, B)>: Functor<B, Target<B> = Self::Target<B>>,
        Self: Sized,
    {
        self.map2(fb, |_, b| b)
    }

    /// Compose two effectful values discarding the result of the second.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x = Some(1);
    /// let y = Some(2);
    /// let actual = x.product_l(y);
    /// assert_eq!(Some(1), actual);
    /// ```
    #[inline]
    fn product_l(self, fb: Self::Target<B>) -> Self
    where
        Self::Target<(Self::Param, B)>: Functor<Self::Param, Target<Self::Param> = Self>,
        Self: Higher<Target<<Self as Higher>::Param> = Self> + Sized,
    {
        self.map2(fb, |a, _| a)
    }
}

impl<T: Semigroupal<B>, B> MapN<B> for T {}
