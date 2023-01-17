//! FlatMap.

use core::marker::PhantomData;

use crate::apply::Apply;
use crate::combinator::id;
use crate::constant1;
use crate::functor::Functor;
use crate::higher::Higher;

/// Gives access to the `flat_map` method. The motivation for separating this out of [Monad] is that
/// there are situations where `flat_map` can be implemented but not `pure`.
pub trait FlatMap<B>: Apply<B> {
    /// Maps a function over a value in the context and flattens the resulting nested context.
    /// This is the same as `self.map(f).flatten()`.
    /// This is also known as `bind` or `>>=` in other languages.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x = Some(1);
    /// let actual = x.flat_map(|x| Some(x.to_string()));
    /// assert_eq!(Some("1".to_string()), actual);
    /// ```
    fn flat_map<F>(self, f: F) -> Self::Target<B>
    where
        F: FnMut(Self::Param) -> Self::Target<B>;

    /// Flattens a nested structure.
    /// This is a convenience method for `flat_map(id)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let actual = Some(Some(1)).flatten();
    /// assert_eq!(Some(1), actual);
    /// ```
    fn flatten(self) -> Self::Target<B>
    where
        Self: FlatMap<B, Param = <Self as Higher>::Target<B>> + Sized,
    {
        self.flat_map(id)
    }

    /// Pair up the value with the result of applying the function to the value.
    ///
    /// # Examples
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x = Some(1);
    /// let actual = x.flat_map(|x| Some(x.to_string()));
    /// assert_eq!(Some("1".to_string()), actual);
    /// ```
    fn m_product<F>(self, mut f: F) -> Self::Target<(Self::Param, B)>
    where
        F: FnMut(Self::Param) -> Self::Target<B>,
        Self: FlatMap<(<Self as Higher>::Param, B)> + Sized,
        Self::Param: Copy,
        Self::Target<B>:
            Functor<(Self::Param, B), Target<(Self::Param, B)> = Self::Target<(Self::Param, B)>>,
    {
        self.flat_map(|a| f(a).map(|b| (a, b)))
    }

    /// `if` lifted into monad.
    ///
    /// # Examples
    /// ```
    /// use rust2fun::constant;
    /// use rust2fun::prelude::*;
    ///
    /// let x = Some(true);
    /// let actual = x.if_m(constant!(Some(1)), constant!(Some(0)));
    /// assert_eq!(Some(1), actual);
    /// ```
    fn if_m<T, F>(self, mut if_true: T, mut if_false: F) -> Self::Target<B>
    where
        T: FnMut() -> Self::Target<B>,
        F: FnMut() -> Self::Target<B>,
        Self: FlatMap<B, Param = bool> + Sized,
    {
        self.flat_map(|x| if x { if_true() } else { if_false() })
    }

    /// Apply a monadic function and discard the result while keeping the effect.
    ///
    /// # Examples
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x = Some(1);
    /// let actual = x.flat_tap(|x| Some(x.to_string()));
    /// assert_eq!(Some(1), actual);
    /// ```
    fn flat_tap<F>(self, mut f: F) -> Self
    where
        F: FnMut(Self::Param) -> Self::Target<B>,
        Self: FlatMap<<Self as Higher>::Param, Target<<Self as Higher>::Param> = Self> + Sized,
        Self::Param: Copy,
        Self::Target<B>: Functor<Self::Param, Target<Self::Param> = Self>,
    {
        fn internal<FA: FlatMap<<FA as Higher>::Param, Target<<FA as Higher>::Param> = FA>>(
            fa: FA,
            g: impl FnMut(FA::Param) -> FA,
        ) -> FA {
            fa.flat_map(g)
        }

        internal(self, |a| f(a).map(constant1!(a)))
    }
}

/// Macro to implement [FlatMap] for types with [Iterator] support.
#[macro_export]
macro_rules! flatmap_iter {
    ($name:ident) => {
        impl<A, B> $crate::flatmap::FlatMap<B> for $name<A>
        {
            #[inline]
            fn flat_map<F>(self, f: F) -> Self::Target<B>
            where
                F: FnMut(A) -> Self::Target<B>,
            {
                self.into_iter().flat_map(f).collect::<$name<B>>()
            }
        }
    };
    ($name:ident, $ct:tt $(+ $dt:tt )*) => {
        impl<A: $ct $(+ $dt )*, B: $ct $(+ $dt )*> $crate::flatmap::FlatMap<B> for $name<A> {
            #[inline]
            fn flat_map<F>(self, f: F) -> Self::Target<B>
            where
                F: FnMut(A) -> Self::Target<B>,
            {
                self.into_iter().flat_map(f).collect::<$name<B>>()
            }
        }
    };
}

impl<A, B> FlatMap<B> for PhantomData<A> {
    #[inline]
    fn flat_map<F>(self, _f: F) -> PhantomData<B>
    where
        F: FnMut(A) -> PhantomData<B>,
    {
        PhantomData
    }
}

impl<A, B> FlatMap<B> for Option<A> {
    #[inline]
    fn flat_map<F>(self, f: F) -> Option<B>
    where
        F: FnMut(A) -> Option<B>,
    {
        self.and_then(f)
    }
}

impl<A, B, E> FlatMap<B> for Result<A, E> {
    #[inline]
    fn flat_map<F>(self, f: F) -> Result<B, E>
    where
        F: FnMut(A) -> Result<B, E>,
    {
        self.and_then(f)
    }
}

if_std! {
    use std::boxed::Box;
    use std::collections::*;
    use std::hash::Hash;
    use std::vec::Vec;

    impl<A, B> FlatMap<B> for Box<A> {
        #[inline]
        fn flat_map<F>(self, mut f: F) -> Box<B>
        where
            F: FnMut(A) -> Box<B>,
        {
            f(*self)
        }
    }

    flatmap_iter!(Vec);
    flatmap_iter!(LinkedList);
    flatmap_iter!(VecDeque);
    flatmap_iter!(BinaryHeap, Ord);
    flatmap_iter!(BTreeSet, Ord);
    flatmap_iter!(HashSet, Eq + Hash);

    impl<A, B, K: Eq + Hash> FlatMap<B> for HashMap<K, A> {
        #[inline]
        fn flat_map<F>(self, mut f: F) -> HashMap<K, B>
        where
            F: FnMut(A) -> HashMap<K, B>,
        {
            self.into_iter().flat_map(|(_, v)|  f(v)).collect()
        }
    }
}
