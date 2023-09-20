//! Bifunctor is a type constructor that takes two type arguments and is a functor in both
//! arguments.

use crate::higher::Higher2;

/// Bifunctor takes two type parameters instead of one, and is a functor in both of these
/// parameters. It defines a function bimap, which allows for mapping over both arguments at the
/// same time
pub trait Bifunctor<C, D>: Higher2 {
    /// Transform a `Self<A, B>` into a `Self<C, D>` by providing a transformation from `A` to `C`
    /// and from 'B' to 'D'
    fn bimap(
        self,
        f: impl FnMut(Self::Param1) -> C,
        g: impl FnMut(Self::Param2) -> D,
    ) -> Self::Target<C, D>;
}

impl<A, B, C, D> Bifunctor<C, D> for Result<A, B> {
    fn bimap(self, mut f: impl FnMut(A) -> C, mut g: impl FnMut(B) -> D) -> Result<C, D> {
        match self {
            Ok(x) => Ok(f(x)),
            Err(e) => Err(g(e)),
        }
    }
}

impl<A, B, C, D> Bifunctor<C, D> for (A, B) {
    fn bimap(self, mut f: impl FnMut(A) -> C, mut g: impl FnMut(B) -> D) -> (C, D) {
        (f(self.0), g(self.1))
    }
}

if_std! {
    use std::collections::HashMap;
    use std::hash::Hash;

    impl<A, B, C: Eq+Hash, D> Bifunctor<C, D> for HashMap<A, B> {
        fn bimap(
            self,
            mut f: impl FnMut(A) -> C,
            mut g: impl FnMut(B) -> D,
        ) -> HashMap<C, D> {
                self.into_iter().map(|(k, v)| (f(k), g(v))).collect()
        }
    }
}
