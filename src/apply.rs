//! Apply.

use core::marker::PhantomData;

use crate::functor::Functor;

/// Weaker version of Applicative has apply but not pure.
pub trait Apply<A, B>: Functor<B> {
    /// Apply a function in a context to a value in a context.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let ff = Some(|x: i32| x.to_string());
    /// assert_eq!(Some("1".to_string()), ff.ap(Some(1)));
    /// assert_eq!(Some("2".to_string()), ff.ap(Some(2)));
    ///
    /// let ff = vec![|x| x + 1, |x| x + 2];
    /// let fa = vec![3, 4];
    /// let actual = ff.ap(fa);
    /// assert_eq!(actual, [4, 5, 5, 6]);
    /// ```
    fn ap(self, fa: Self::Target<A>) -> Self::Target<B>
    where
        Self::Param: FnMut(A) -> B;
}

/// Macro to implement [Apply] for types with [Iterator] support.
#[macro_export]
macro_rules! apply_iter {
    ($name:ident) => {
        impl<F, A: Clone, B> $crate::apply::Apply<A, B> for $name<F> {
            #[inline]
            fn ap(self, fa: Self::Target<A>) -> Self::Target<B>
            where
                Self::Param: FnMut(A) -> B,
            {
                self.into_iter()
                    .flat_map(|mut f| fa.clone().into_iter().map(move |a| f(a)))
                    .collect::<$name<B>>()
            }
        }
    };
    ($name:ident, $ct:tt $(+ $dt:tt )*) => {
        impl<F: $ct $(+ $dt )*, A: Clone, B: $ct $(+ $dt )*> $crate::apply::Apply<A, B> for $name<F> {
            #[inline]
            fn ap(self, fa: Self::Target<A>) -> Self::Target<B>
            where
                Self::Param: FnMut(A) -> B,
            {
                self.into_iter()
                    .flat_map(|mut f| fa.clone().into_iter().map(move |a| f(a)))
                    .collect::<$name<B>>()
            }
        }
    };
}

impl<F, A, B> Apply<A, B> for PhantomData<F> {
    #[inline]
    fn ap(self, _fa: PhantomData<A>) -> PhantomData<B>
    where
        F: FnOnce(A) -> B,
    {
        PhantomData
    }
}

impl<F, A, B> Apply<A, B> for Option<F> {
    #[inline]
    fn ap(self, fa: Option<A>) -> Option<B>
    where
        F: FnOnce(A) -> B,
    {
        match (self, fa) {
            (Some(f), Some(a)) => Some(f(a)),
            _ => None,
        }
    }
}

impl<F, A, B, E> Apply<A, B> for Result<F, E> {
    #[inline]
    fn ap(self, fa: Result<A, E>) -> Result<B, E>
    where
        F: FnOnce(A) -> B,
    {
        match (self, fa) {
            (Ok(f), Ok(a)) => Ok(f(a)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
}

if_std! {
    use std::boxed::Box;
    use std::collections::*;
    use std::hash::Hash;
    use std::vec::Vec;

    impl<F, A, B> Apply<A, B> for Box<F> {
        #[inline]
        fn ap(self, fa: Box<A>) -> Box<B>
        where
            F: FnOnce(A) -> B,
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

    impl<F, A, B, K: Eq + Hash> Apply<A, B> for HashMap<K, F> {
        #[inline]
        fn ap(mut self, fa: HashMap<K, A>) -> HashMap<K, B>
        where
            F: FnOnce(A) -> B,
        {
            fa.into_iter()
                .filter_map(|(k, a)| self.remove(&k).map(|f| (k, f(a))))
                .collect()
        }
    }
}
