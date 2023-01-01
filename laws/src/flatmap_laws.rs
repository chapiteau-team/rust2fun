use rust2fun::prelude::*;

use crate::is_eq::IsEq;

pub fn flat_map_associativity<FA, B, C, F, G>(fa: FA, f: F, g: G) -> IsEq<FA::Target<C>>
where
    FA: FlatMap<B> + FlatMap<C> + Clone,
    F: Fn(FA::Param) -> FA::Target<B>,
    G: Fn(B) -> FA::Target<C> + Clone,
    FA::Target<B>: FlatMap<C, Target<C> = FA::Target<C>>,
    FA::Target<C>: Eq,
{
    let lhs = fa.clone().flat_map(|a| f(a).flat_map(g.clone()));
    let rhs = fa.flat_map(f).flat_map(g);

    IsEq::equal_under_law(lhs, rhs)
}

pub fn flat_map_consistent_apply<FA, B, F>(fa: FA, fab: FA::Target<F>) -> IsEq<FA::Target<B>>
where
    FA: Apply<B> + Clone,
    F: Fn(FA::Param) -> B,
    FA::Target<F>: FlatMap<B, Target<B> = FA::Target<B>> + Clone,
    FA::Target<B>: Eq,
{
    let lhs = fab.clone().flat_map(|f: F| fa.clone().map(f));
    let rhs = fa.ap(fab);

    IsEq::equal_under_law(lhs, rhs)
}

pub fn m_product_consistency<FA, B, F>(fa: FA, mut f: F) -> IsEq<FA::Target<(FA::Param, B)>>
where
    FA: FlatMap<B> + FlatMap<(<FA as Higher>::Param, B)> + Clone,
    FA::Param: Clone,
    F: FnMut(FA::Param) -> FA::Target<B>,
    FA::Target<B>: Functor<(FA::Param, B), Target<(FA::Param, B)> = FA::Target<(FA::Param, B)>>,
    FA::Target<(FA::Param, B)>: Eq,
{
    let rhs = fa
        .clone()
        .flat_map(|a| f(a.clone()).map(|b| (a.clone(), b)));
    let lhs = fa.m_product(f);

    IsEq::equal_under_law(lhs, rhs)
}
