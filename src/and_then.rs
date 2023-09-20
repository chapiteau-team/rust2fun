//! AndThen.

use std::marker::PhantomData;

use crate::higher::Higher;

/// Gives access to the [and_then] method. This trait is needed to implement [ApN]. The motivation
/// for not using [FlatMap] is that there are situations where `and_then` can be implemented but
/// not [FlatMap::flat_map], e.g. [Validated].
///
/// [and_then]: AndThen::and_then
/// [ApN]: crate::ap_n::ApN
/// [Validated]: crate::data::validated::Validated
pub trait AndThen<B>: Higher {
    /// Maps a function over a value in the context and flattens the resulting nested context.
    fn and_then<F>(self, f: F) -> Self::Target<B>
    where
        F: FnMut(Self::Param) -> Self::Target<B>;
}

// TODO. Refactor this when specialization is stable.
/// Macro to implement [AndThen] for types implementing a [FlatMap::flat_map].
#[macro_export]
macro_rules! and_then_flat_map {
    ($name:ident<$( $t:tt ),+>) => {
        impl<B, $( $t ),+> $crate::and_then::AndThen<B> for $name<$( $t ),+> {
            #[inline]
            fn and_then<F>(self, f: F) -> Self::Target<B>
            where
                F: FnMut(Self::Param) -> Self::Target<B>,
            {
                $crate::flatmap::FlatMap::flat_map(self, f)
            }
        }
    };
    ($name:ident<$( $t:tt ),+>, $ct:tt $(+ $dt:tt )*) => {
        impl<B:$ct $(+ $dt )*, $( $t ),+> $crate::and_then::AndThen<B> for $name<$( $t ),+> {
            #[inline]
            fn and_then<F>(self, f: F) -> Self::Target<B>
            where
                F: FnMut(Self::Param) -> Self::Target<B>,
            {
                $crate::flatmap::FlatMap::flat_map(self, f)
            }
        }
    };
}

impl<A, B> AndThen<B> for PhantomData<A> {
    fn and_then<F>(self, _f: F) -> PhantomData<B>
    where
        F: FnMut(A) -> PhantomData<B>,
    {
        PhantomData
    }
}

and_then_flat_map!(Option<T>);
and_then_flat_map!(Result<T, E>);

if_std! {
    use std::boxed::Box;
    use std::collections::*;
    use std::hash::Hash;
    use std::vec::Vec;
    use crate::flatmap::FlatMap;

    and_then_flat_map!(Vec<T>);
    and_then_flat_map!(LinkedList<T>);
    and_then_flat_map!(VecDeque<T>);
    and_then_flat_map!(Box<T>);
    and_then_flat_map!(BinaryHeap<T>, Ord);
    and_then_flat_map!(BTreeSet<T>, Ord);
    and_then_flat_map!(HashSet<T>, Hash + Eq);

    impl<A, B, K: Hash + Eq> AndThen<B> for HashMap<K, A> {
        #[inline]
        fn and_then<F>(self, f: F) -> HashMap<K, B>
        where
            F: FnMut(A) -> HashMap<K, B>,
        {
            self.flat_map(f)
        }
    }
}
