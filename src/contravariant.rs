//! Contravariant functors.

use core::marker::PhantomData;

use crate::invariant::Invariant;

/// Lift a function f to operate on Contravariant functors.
///
/// # Examples
///
/// ```
/// use std::marker::PhantomData;
/// use rust2fun::prelude::*;
///
/// let f = lift_contravariant(|x: i32| x.to_string());
/// assert_eq!(PhantomData::<i32>, f(PhantomData::<String>));
/// ```
pub fn lift_contravariant<FA, B>(f: impl FnMut(B) -> FA::Param) -> impl FnOnce(FA) -> FA::Target<B>
where
    FA: Contravariant<B>,
{
    |fa: FA| fa.contramap(f)
}

/// Contravariant functor.
pub trait Contravariant<B>: Invariant<B> {
    /// Transform a `Self<A>` into a `Self<B>` by providing a transformation from `B` to `A`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::marker::PhantomData;
    /// use rust2fun::prelude::*;
    ///
    /// let x = PhantomData::<i32>;
    /// let actual = x.contramap(|x: String| x.parse::<i32>().unwrap());
    /// assert_eq!(PhantomData::<String>, actual);
    /// ```
    fn contramap(self, f: impl FnMut(B) -> Self::Param) -> Self::Target<B>;
}

impl<A, B> Contravariant<B> for PhantomData<A> {
    #[inline]
    fn contramap(self, _f: impl FnMut(B) -> Self::Param) -> PhantomData<B> {
        PhantomData::<B>
    }
}
