//! Apply.

use core::marker::PhantomData;

use crate::functor::Functor;
use crate::prelude::Higher;
use crate::semigroupal::Semigroupal;

/// Weaker version of Applicative has apply but not pure.
pub trait Apply<B>: Functor<B> + Semigroupal<B> {
    /// Apply a function in a context to a value in a context.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let f = Some(|x: i32| x.to_string());
    /// let x = Some(1);
    /// let actual = x.ap(f);
    /// assert_eq!(Some("1".to_string()), actual);
    /// ```
    fn ap<F>(self, ff: Self::Target<F>) -> Self::Target<B>
    where
        F: FnMut(Self::Param) -> B;

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
    fn map2<Z, F>(self, fb: Self::Target<B>, mut f: F) -> Self::Target<Z>
    where
        F: FnMut(Self::Param, B) -> Z,
        Self::Target<(Self::Param, B)>: Functor<Z, Target<Z> = Self::Target<Z>>,
        Self: Sized,
    {
        self.product(fb).map(|(a, b)| f(a, b))
    }

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
    fn product_l(self, fb: Self::Target<B>) -> Self
    where
        Self::Target<(Self::Param, B)>: Functor<Self::Param, Target<Self::Param> = Self>,
        Self: Higher<Target<<Self as Higher>::Param> = Self> + Sized,
    {
        self.map2(fb, |a, _| a)
    }
}

/// Macro to implement [Apply] for types with [Iterator] support.
#[macro_export]
macro_rules! apply_iter {
    ($name:ident) => {
        impl<A, B> $crate::apply::Apply<B> for $name<A> {
            #[inline]
            fn ap<F>(self, ff: Self::Target<F>) -> Self::Target<B>
            where
                F: FnMut(A) -> B,
            {
                self.into_iter().zip(ff).map(|(a, mut f)| f(a)).collect::<$name<B>>()
            }
        }
    };
    ($name:ident, $ct:tt $(+ $dt:tt )*) => {
        impl<A: $ct $(+ $dt )*, B: $ct $(+ $dt )*> $crate::apply::Apply<B> for $name<A> {
            #[inline]
            fn ap<F>(self, ff: Self::Target<F>) -> Self::Target<B>
            where
                F: FnMut(A) -> B,
            {
                self.into_iter().zip(ff).map(|(a, mut f)| f(a)).collect::<$name<B>>()
            }
        }
    };
}

impl<A, B> Apply<B> for PhantomData<A> {
    #[inline]
    fn ap<F>(self, _ff: PhantomData<F>) -> PhantomData<B>
    where
        F: FnMut(A) -> B,
    {
        PhantomData::<B>
    }
}

impl<A, B> Apply<B> for Option<A> {
    #[inline]
    fn ap<F>(self, ff: Option<F>) -> Option<B>
    where
        F: FnMut(A) -> B,
    {
        self.and_then(|a| ff.map(|mut f: F| f(a)))
    }
}

impl<A, B, E> Apply<B> for Result<A, E> {
    #[inline]
    fn ap<F>(self, ff: Result<F, E>) -> Result<B, E>
    where
        F: FnMut(A) -> B,
    {
        self.and_then(|a| ff.map(|mut f: F| f(a)))
    }
}

if_std! {
    use std::boxed::Box;
    use std::collections::*;
    use std::hash::Hash;
    use std::vec::Vec;

    impl<A, B> Apply<B> for Box<A> {
        #[inline]
        fn ap<F>(self, mut ff: Box<F>) -> Box<B>
        where
            F: FnMut(A) -> B,
        {
            Box::new((*ff)(*self))
        }
    }

    apply_iter!(Vec);
    apply_iter!(LinkedList);
    apply_iter!(VecDeque);
    apply_iter!(BinaryHeap, Ord);
    apply_iter!(BTreeSet, Ord);
    apply_iter!(HashSet, Eq + Hash);
}
