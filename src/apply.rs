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
    /// assert_eq!(Some("1".to_string()), f.ap(Some(1)));
    /// assert_eq!(Some("2".to_string()), f.ap(Some(2)));
    /// ```
    fn ap<A>(self, fa: Self::Target<A>) -> Self::Target<B>
    where
        Self::Param: FnOnce(A) -> B;

    /// Is a binary version of [ap].
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let f = Some(|x, y| x + y);
    /// assert_eq!(Some(3), f.ap2(Some(1), Some(2)));
    /// ```
    fn ap2<A, Z>(self, fa: Self::Target<A>, fb: Self::Target<B>) -> Self::Target<Z>
    where
        Self::Target<(Self::Param, B)>: Semigroupal<A, Target<A> = Self::Target<A>>
            + Higher<
                Target<(<Self::Target<(Self::Param, B)> as Higher>::Param, A)> = Self::Target<(
                    <Self::Target<(Self::Param, B)> as Higher>::Param,
                    A,
                )>,
            >,
        Self::Target<(<Self::Target<(Self::Param, B)> as Higher>::Param, A)>:
            Functor<Z, Target<Z> = Self::Target<Z>>,
        Self: Sized,
        Self::Param: FnOnce(A, B) -> Z,
    {
        self.product(fb).product(fa).map(|((f, b), a)| f(a, b))
    }

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

/// Macro to implement [Apply] for types with [Iterator] support.
#[macro_export]
macro_rules! apply_iter {
    ($name:ident) => {
        impl<F, B> $crate::apply::Apply<B> for $name<F> {
            #[inline]
            fn ap<A>(self, fa: Self::Target<A>) -> Self::Target<B>
            where
                Self::Param: FnOnce(A) -> B,
            {
                self.into_iter()
                    .zip(fa)
                    .map(|(f, a)| f(a))
                    .collect::<$name<B>>()
            }
        }
    };
    ($name:ident, $ct:tt $(+ $dt:tt )*) => {
        impl<F: $ct $(+ $dt )*, B: $ct $(+ $dt )*> $crate::apply::Apply<B> for $name<F> {
            #[inline]
            fn ap<A>(self, fa: Self::Target<A>) -> Self::Target<B>
            where
                Self::Param: FnOnce(A) -> B,
            {
                self.into_iter()
                    .zip(fa)
                    .map(|(f, a)| f(a))
                    .collect::<$name<B>>()
            }
        }
    };
}

impl<F, B> Apply<B> for PhantomData<F> {
    #[inline]
    fn ap<A>(self, _fa: PhantomData<A>) -> PhantomData<B>
    where
        Self::Param: FnOnce(A) -> B,
    {
        PhantomData
    }
}

impl<F, B> Apply<B> for Option<F> {
    #[inline]
    fn ap<A>(self, fa: Option<A>) -> Option<B>
    where
        Self::Param: FnOnce(A) -> B,
    {
        self.and_then(|f| fa.map(|a| f(a)))
    }
}

impl<F, B, E> Apply<B> for Result<F, E> {
    #[inline]
    fn ap<A>(self, fa: Result<A, E>) -> Result<B, E>
    where
        Self::Param: FnOnce(A) -> B,
    {
        self.and_then(|f| fa.map(|a| f(a)))
    }
}

if_std! {
    use std::boxed::Box;
    use std::collections::*;
    use std::hash::Hash;
    use std::vec::Vec;

    impl<F, B> Apply<B> for Box<F> {
        #[inline]
        fn ap<A>(self, fa: Box<A>) -> Box<B>
        where
            Self::Param: FnOnce(A) -> B,
        {
            Box::new((*self)(*fa))
        }
    }

    apply_iter!(Vec);
    apply_iter!(LinkedList);
    apply_iter!(VecDeque);
    apply_iter!(BinaryHeap, Ord);
    apply_iter!(BTreeSet, Ord);
    apply_iter!(HashSet, Eq + Hash);

    impl<F, B, K: Eq + Hash> Apply<B> for HashMap<K, F> {
        #[inline]
        fn ap<A>(mut self, fa: HashMap<K, A>) -> HashMap<K, B>
        where
            Self::Param: FnOnce(A) -> B,
        {
            fa.into_iter()
                .filter_map(|(k, a)| self.remove(&k).map(|f| (k, f(a))))
                .collect()
        }
    }
}
