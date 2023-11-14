//! Functor transformation.

use crate::higher::Higher;
use crate::monoid::Monoid;
use crate::pure::Pure;
use core::marker::PhantomData;

/// Functor transformation from `A` to `B`. It transforms values from one first-order-kinded type
/// (a type that takes a single type parameter, such as `Vec` or `Option`)
/// into another first-order-kinded type. This transformation is universal, meaning that
/// a `Fnk<Vec<T>, Option<T>>` will translate all `Vec<T>` values into an `Option<T>` value
/// for all possible types of `T`.
///
/// # Examples
///
/// ```
/// use rust2fun::prelude::*;
///
/// fn foo<T, E>(a: Vec<Result<T, E>>, f: impl FnK<Result<T, E>, Option<T>>) -> Vec<Option<T>> {
///    a.into_iter().map(|x| f.apply(x)).collect()
/// }
///
/// assert_eq!(vec![Some(1), None], foo(vec![Ok(1), Err(2)], Result::ok));
/// ```
pub trait FnK<A, B>
where
    A: Higher,
    B: Higher<Param = A::Param>,
{
    /// Applies this functor transformation from `A` to `B`.
    fn apply(&self, a: A) -> B;

    /// Composes this functor transformation with another functor transformation.
    /// This transformation will be applied to the result of the provided transformation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// assert_eq!(Some(2), Result::ok.compose(NthToResult(1, "err")).apply(vec![1, 2]));
    /// assert_eq!(None::<u8>, Result::ok.compose(NthToResult(1, "err")).apply(vec![]));
    /// ```
    fn compose<Z, F>(self, f: F) -> Composition<Z, A, B, F, Self>
    where
        Z: Higher,
        A: Higher<Param = Z::Param>,
        F: FnK<Z, A>,
        Self: Sized,
    {
        Composition {
            f,
            g: self,
            _phantom: PhantomData,
        }
    }

    /// Composes this functor transformation with another functor transformation.
    /// This transformation will be applied first.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// assert_eq!(Some(2), NthToResult(1, "err").and_then(Result::ok).apply(vec![1, 2]));
    /// assert_eq!(None::<u8>, NthToResult(1, "err").and_then(Result::ok).apply(vec![]));
    /// ```
    fn and_then<C, F>(self, f: F) -> Composition<A, B, C, Self, F>
    where
        C: Higher<Param = B::Param>,
        F: FnK<B, C>,
        Self: Sized,
    {
        f.compose(self)
    }
}

/// Functor transformation from `A` to `C` by composing two functor transformations.
/// This transformation will apply the first transformation to the input value and then
/// apply the second transformation to the result of the first transformation.
pub struct Composition<A, B, C, F, G>
where
    A: Higher,
    B: Higher<Param = A::Param>,
    C: Higher<Param = B::Param>,
    F: FnK<A, B>,
    G: FnK<B, C>,
{
    f: F,
    g: G,
    _phantom: PhantomData<(A, B, C)>,
}

impl<A, B, C, F, G> FnK<A, C> for Composition<A, B, C, F, G>
where
    A: Higher,
    B: Higher<Param = A::Param>,
    C: Higher<Param = B::Param>,
    F: FnK<A, B>,
    G: FnK<B, C>,
{
    #[inline]
    fn apply(&self, a: A) -> C {
        self.g.apply(self.f.apply(a))
    }
}

/// Functor transformation from `IntoIterator` implementer to `Option`.
/// This transformation will take the first element of the iterator and return it as an `Option`.
/// If the iterator is empty, it will return `None`.
///
/// # Examples
///
/// ```
/// use rust2fun::prelude::*;
///
/// assert_eq!(Some(1), FirstToOption.apply(vec![1, 2, 3]));
/// assert_eq!(None, FirstToOption.apply(Vec::<i32>::new()));
/// ```
pub struct FirstToOption;
impl<T, A> FnK<A, Option<T>> for FirstToOption
where
    A: IntoIterator<Item = T> + Higher<Param = T>,
{
    #[inline]
    fn apply(&self, a: A) -> Option<T> {
        a.into_iter().next()
    }
}

/// Functor transformation from `IntoIterator` implementer to `Option`.
/// This transformation will take the last element of the iterator and return it as an `Option`.
/// If the iterator is empty, it will return `None`.
///
/// # Examples
///
/// ```
/// use rust2fun::prelude::*;
///
/// assert_eq!(Some(3), LastToOption.apply(vec![1, 2, 3]));
/// assert_eq!(None, LastToOption.apply(Vec::<i32>::new()));
/// ```
pub struct LastToOption;
impl<T, A> FnK<A, Option<T>> for LastToOption
where
    A: IntoIterator<Item = T> + Higher<Param = T>,
{
    #[inline]
    fn apply(&self, a: A) -> Option<T> {
        a.into_iter().last()
    }
}

/// Functor transformation from `IntoIterator` implementer to `Option`.
/// This transformation will take the nth element of the iterator and return it as an `Option`.
/// If the iterator is empty or has less than `N` elements, it will return `None`.
///
/// # Examples
///
/// ```
/// use rust2fun::prelude::*;
///
/// assert_eq!(Some(2), NthToOption(1).apply(vec![1, 2, 3]));
/// assert_eq!(None, NthToOption(1).apply(vec![1]));
/// assert_eq!(None, NthToOption(1).apply(Vec::<i32>::new()));
/// ```
pub struct NthToOption(pub usize);
impl<T, A> FnK<A, Option<T>> for NthToOption
where
    A: IntoIterator<Item = T> + Higher<Param = T>,
{
    #[inline]
    fn apply(&self, a: A) -> Option<T> {
        a.into_iter().nth(self.0)
    }
}

/// Functor transformation from `IntoIterator` implementer to `Result`.
/// This transformation will take the first element of the iterator and return it as an `Result`.
/// If the iterator is empty, it will return `Err(E)`.
/// This transformation is similar to [FirstToOption], but it allows you to specify the error
/// value.
///
/// # Examples
///
/// ```
/// use rust2fun::prelude::*;
///
/// assert_eq!(Ok(1), FirstToResult("err").apply(vec![1, 2, 3]));
/// assert_eq!(Err("err"), FirstToResult("err").apply(Vec::<i32>::new()));
/// ```
pub struct FirstToResult<E>(pub E);
impl<T, E: Clone, A> FnK<A, Result<T, E>> for FirstToResult<E>
where
    A: IntoIterator<Item = T> + Higher<Param = T>,
{
    #[inline]
    fn apply(&self, a: A) -> Result<T, E> {
        a.into_iter().next().ok_or_else(|| self.0.clone())
    }
}

/// Functor transformation from `IntoIterator` implementer to `Result`.
/// This transformation will take the last element of the iterator and return it as an `Result`.
/// If the iterator is empty, it will return `Err(E)`.
/// This transformation is similar to [LastToOption], but it allows you to specify the error
/// value.
///
/// # Examples
///
/// ```
/// use rust2fun::prelude::*;
///
/// assert_eq!(Ok(3), LastToResult("err").apply(vec![1, 2, 3]));
/// assert_eq!(Err("err"), LastToResult("err").apply(Vec::<i32>::new()));
/// ```
pub struct LastToResult<E>(pub E);
impl<T, E: Clone, A> FnK<A, Result<T, E>> for LastToResult<E>
where
    A: IntoIterator<Item = T> + Higher<Param = T>,
{
    #[inline]
    fn apply(&self, a: A) -> Result<T, E> {
        a.into_iter().last().ok_or_else(|| self.0.clone())
    }
}

/// Functor transformation from `IntoIterator` implementer to `Result`.
/// This transformation will take the nth element of the iterator and return it as an `Result`.
/// If the iterator is empty or has less than `N` elements, it will return `Err(E)`.
/// This transformation is similar to [NthToOption], but it allows you to specify the error
/// value.
///
/// # Examples
///
/// ```
/// use rust2fun::prelude::*;
///
/// assert_eq!(Ok(2), NthToResult(1, "err").apply(vec![1, 2, 3]));
/// assert_eq!(Err("err"), NthToResult(1, "err").apply(vec![1]));
/// assert_eq!(Err("err"), NthToResult(1, "err").apply(Vec::<i32>::new()));
/// ```
pub struct NthToResult<E>(pub usize, pub E);
impl<T, E: Clone, A> FnK<A, Result<T, E>> for NthToResult<E>
where
    A: IntoIterator<Item = T> + Higher<Param = T>,
{
    #[inline]
    fn apply(&self, a: A) -> Result<T, E> {
        a.into_iter().nth(self.0).ok_or_else(|| self.1.clone())
    }
}

impl<A, B, F> FnK<A, B> for F
where
    A: Higher,
    B: Higher<Param = A::Param>,
    F: Fn(A) -> B,
{
    #[inline]
    fn apply(&self, a: A) -> B {
        self(a)
    }
}

if_std! {
    use std::vec;
    use std::vec::Vec;

    /// Functor transformation from `Option` to `Vec`.
    /// This transformation will return a `Vec` with one element if the `Option` is `Some`.
    /// If the `Option` is `None`, it will return an empty `Vec`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// assert_eq!(vec![1], OptionToVec.apply(Some(1)));
    /// assert_eq!(Vec::<i32>::new(), OptionToVec.apply(None));
    /// ```
    pub struct OptionToVec;
    impl<T> FnK<Option<T>, Vec<T>> for OptionToVec {
        #[inline]
        fn apply(&self, a: Option<T>) -> Vec<T> {
            match a {
                Some(x) => vec![x],
                None => Vec::new(),
            }
        }
    }

    /// Functor transformation from `Result` to `Vec`.
    /// This transformation will return a `Vec` with one element if the `Result` is `Ok`.
    /// If the `Result` is `Err`, it will return an empty `Vec`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// assert_eq!(vec![1], ResultToVec.apply(Ok::<_, ()>(1)));
    /// assert_eq!(Vec::<i32>::new(), ResultToVec.apply(Err(1)));
    /// ```
    pub struct ResultToVec;
    impl<T, E> FnK<Result<T, E>, Vec<T>> for ResultToVec {
        #[inline]
        fn apply(&self, a: Result<T, E>) -> Vec<T> {
            match a {
                Ok(x) => vec![x],
                Err(_) => Vec::new(),
            }
        }
    }
}

/// Functor transformation from `Option` to a type implementing [Pure] and [Monoid].
/// This transformation will return the value inside the `Option` if it is `Some`.
/// If the `Option` is `None`, it will return the empty value of the target type.
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
/// use rust2fun::prelude::*;
///
/// fn foo<T, F: Higher<Param = T>>(a: Option<T>, f: impl FnK<Option<T>, F>) -> F {
///    f.apply(a)
/// }
///
/// assert_eq!(HashSet::pure(1), foo(Some(1), OptionToF));
/// ```
pub struct OptionToF;
impl<T, F> FnK<Option<T>, F> for OptionToF
where
    F: Pure<Param = T> + Monoid,
{
    #[inline]
    fn apply(&self, a: Option<T>) -> F {
        match a {
            Some(x) => F::pure(x),
            None => F::empty(),
        }
    }
}
