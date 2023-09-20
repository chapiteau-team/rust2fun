//! Semigroupal.

use core::marker::PhantomData;

use crate::higher::Higher;

/// Semigroupal captures the idea of composing independent effectful values.
///
/// [Semigroupal]s are associative under the bijection `f = (a,(b,c)) -> ((a,b),c)` or
/// `f = ((a,b),c) -> (a,(b,c))`.
pub trait Semigroupal<B>: Higher {
    /// Combine two effectful values into a single effectful value maintaining the effects of both.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let fa = Some(1);
    /// let fb = Some("1");
    /// let actual = fa.product(fb);
    /// assert_eq!(Some((1, "1")), actual);
    ///
    /// let fa = vec![1, 2];
    /// let fb = vec![3, 4];
    /// let actual = fa.product(fb);
    /// assert_eq!(vec![(1, 3), (1, 4), (2, 3), (2, 4)], actual);
    /// ```
    fn product(self, fb: Self::Target<B>) -> Self::Target<(Self::Param, B)>;
}

/// Macro to implement [Semigroupal] for types with [Iterator] support.
#[macro_export]
macro_rules! semigroupal_iter {
    ($name:ident) => {
        impl<A: Clone, B: Clone> $crate::semigroupal::Semigroupal<B> for $name<A> {
            #[inline]
            fn product(self, fb: Self::Target<B>) -> Self::Target<(A, B)> {
                self.into_iter()
                    .flat_map(|a| fb.clone().into_iter().map(move |b| (a. clone(), b)))
                    .collect()
            }
        }
    };
    ($name:ident, $ct:tt $(+ $dt:tt )*) => {
        impl<A: Clone + $ct $(+ $dt )*, B: Clone + $ct $(+ $dt )*> $crate::semigroupal::Semigroupal<B> for $name<A> {
            #[inline]
            fn product(self, fb: Self::Target<B>) -> Self::Target<(A, B)> {
                self.into_iter()
                    .flat_map(|a| fb.clone().into_iter().map(move |b| (a.clone(), b)))
                    .collect()
            }
        }
    };
}

impl<A, B> Semigroupal<B> for PhantomData<A> {
    #[inline]
    fn product(self, _fb: PhantomData<B>) -> PhantomData<(A, B)> {
        PhantomData::<(A, B)>
    }
}

impl<A, B> Semigroupal<B> for Option<A> {
    #[inline]
    fn product(self, fb: Option<B>) -> Option<(A, B)> {
        match (self, fb) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
}

impl<A, B, E> Semigroupal<B> for Result<A, E> {
    #[inline]
    fn product(self, fb: Result<B, E>) -> Result<(A, B), E> {
        match (self, fb) {
            (Ok(a), Ok(b)) => Ok((a, b)),
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

    impl<A, B> Semigroupal<B> for Box<A> {
        #[inline]
        fn product(self, fb: Box<B>) -> Box<(A, B)> {
            Box::new((*self, *fb))
        }
    }

    semigroupal_iter!(Vec);
    semigroupal_iter!(LinkedList);
    semigroupal_iter!(VecDeque);
    semigroupal_iter!(BinaryHeap, Ord);
    semigroupal_iter!(BTreeSet, Ord);
    semigroupal_iter!(HashSet, Eq + Hash);

    impl<A, B, K: Eq + Hash> Semigroupal<B> for HashMap<K, A> {
        #[inline]
        fn product(self, mut fb: HashMap<K, B>) -> HashMap<K, (A, B)> {
            self.into_iter()
                .filter_map(|(k, a)| fb.remove(&k).map(|b| (k, (a, b))))
                .collect()
        }
    }
}
