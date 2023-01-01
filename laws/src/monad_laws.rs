use rust2fun::prelude::*;

use crate::is_eq::IsEq;

pub fn monad_left_identity<FA, B, F>(a: FA::Param, mut f: F) -> IsEq<FA::Target<B>>
where
    FA: Monad<B>,
    FA::Param: Clone,
    F: FnMut(FA::Param) -> FA::Target<B>,
    FA::Target<B>: Eq,
{
    let lhs = f(a.clone());
    let rhs = FA::pure(a).flat_map(f);

    IsEq::equal_under_law(lhs, rhs)
}

pub fn monad_right_identity<FA>(fa: FA) -> IsEq<FA>
where
    FA: Monad<<FA as Higher>::Param, Target<<FA as Higher>::Param> = FA> + Eq + Clone,
{
    let lhs = fa.clone();
    let rhs = fa.flat_map(FA::pure);

    IsEq::equal_under_law(lhs, rhs)
}

pub fn map_flat_map_coherence<FA, B, F>(fa: FA, mut f: F) -> IsEq<FA::Target<B>>
where
    FA: Monad<B> + Clone,
    F: FnMut(FA::Param) -> B,
    FA::Target<B>: Applicative + Eq,
{
    let lhs = fa.clone().flat_map(|a| <FA::Target<B>>::pure(f(a)));
    let rhs = fa.map(f);

    IsEq::equal_under_law(lhs, rhs)
}
