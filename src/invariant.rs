//! Invariant functors.

use core::marker::PhantomData;

use crate::functor::Functor;
use crate::higher::Higher;

/// Invariant functor (also known as exponential functor).
pub trait Invariant<B>: Higher {
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
    fn imap<F, G>(self, f: F, g: G) -> Self::Target<B>
    where
        F: FnMut(Self::Param) -> B,
        G: FnMut(B) -> Self::Param;
}

/// Macro to implement [Invariant] for types implementing [Functor].
#[macro_export]
macro_rules! invariant_functor {
    ($name:ident<$( $t:tt ),+>) => {
        impl<B, $( $t ),+> $crate::invariant::Invariant<B> for $name<$( $t ),+> {
            #[inline]
            fn imap<F, G>(self, f: F, _g: G) -> Self::Target<B>
            where
                F: FnMut(Self::Param) -> B,
                G: FnMut(B) -> Self::Param,
            {
                self.fmap(f)
            }
        }
    };
    ($name:ident<$( $t:tt ),+>, $ct:tt $(+ $dt:tt )*) => {
        impl<B:$ct $(+ $dt )*, $( $t ),+> $crate::invariant::Invariant<B> for $name<$( $t ),+> {
            #[inline]
            fn imap<F, G>(self, f: F, _g: G) -> Self::Target<B>
            where
                F: FnMut(Self::Param) -> B,
                G: FnMut(B) -> Self::Param,
            {
                self.fmap(f)
            }
        }
    };
}

/// Macro to implement [Invariant] for types implementing a contravariant [Functor].
#[macro_export]
macro_rules! invariant_contravariant {
    ($name:ident<$( $t:tt ),+>) => {
        impl<B, $( $t ),+> $crate::invariant::Invariant<B> for $name<$( $t ),+> {
            #[inline]
            fn imap<F, G>(self, _f: F, g: G) -> Self::Target<B>
            where
                F: FnMut(Self::Param) -> B,
                G: FnMut(B) -> Self::Param,
            {
                self.contramap(g)
            }
        }
    };
    ($name:ident<$( $t:tt ),+>, $ct:tt $(+ $dt:tt )*) => {
        impl<B:$ct $(+ $dt )*, $( $t ),+> $crate::invariant::Invariant<B> for $name<$( $t ),+> {
            #[inline]
            fn imap<F, G>(self, _f: F, g: G) -> Self::Target<B>
            where
                F: FnMut(Self::Param) -> B,
                G: FnMut(B) -> Self::Param,
            {
                self.contramap(g)
            }
        }
    };
}

impl<A, B> Invariant<B> for PhantomData<A> {
    #[inline]
    fn imap<F, G>(self, _f: F, _g: G) -> PhantomData<B>
    where
        F: FnMut(Self::Param) -> B,
        G: FnMut(B) -> Self::Param,
    {
        PhantomData
    }
}

invariant_functor!(Option<T>);
invariant_functor!(Result<T, E>);

if_std! {
    use std::boxed::Box;
    use std::collections::*;
    use std::hash::Hash;
    use std::vec::Vec;

    invariant_functor!(Vec<T>);
    invariant_functor!(LinkedList<T>);
    invariant_functor!(VecDeque<T>);
    invariant_functor!(Box<T>);
    invariant_functor!(BinaryHeap<T>, Ord);
    invariant_functor!(BTreeSet<T>, Ord);
    invariant_functor!(HashSet<T>, Hash + Eq);

    impl<A, B, K: Hash + Eq> Invariant<B> for HashMap<K, A> {
        #[inline]
        fn imap<F, G>(self, mut f: F, _g: G) -> HashMap<K, B>
        where
            F: FnMut(Self::Param) -> B,
            G: FnMut(B) -> Self::Param,
        {
            self.into_iter().map(|(k, v)| (k, f(v))).collect()
        }
    }
}
