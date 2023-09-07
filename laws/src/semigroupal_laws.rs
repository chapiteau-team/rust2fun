use rust2fun::prelude::*;

use crate::is_eq::IsEq;

pub fn semigroupal_associativity<FA, B, C>(
    fa: FA,
    fb: FA::Target<B>,
    fc: FA::Target<C>,
) -> IsEq<FA::Target<(FA::Param, B, C)>>
where
    FA: Semigroupal<B> + Semigroupal<(B, C)> + Clone,
    FA::Target<(<FA as Higher>::Param, B)>: Semigroupal<C>,
    FA::Target<B>: Semigroupal<C> + Clone,
    FA::Target<C>: Clone,
    <FA::Target<(<FA as Higher>::Param, B)> as Higher>::Target<((FA::Param, B), C)>:
        Invariant<(FA::Param, B, C)>,
    FA::Target<(<FA as Higher>::Param, (B, C))>: Invariant<(FA::Param, B, C)>,
{
    let lhs = fa
        .clone()
        .product(fb.clone())
        .product(fc.clone().unsafe_cast())
        .imap(|((a, b), c)| (a, b, c), |(a, b, c)| ((a, b), c))
        .unsafe_cast();

    let rhs = fa
        .product(fb.product(fc.unsafe_cast()).unsafe_cast())
        .imap(|(a, (b, c))| (a, b, c), |(a, b, c)| (a, (b, c)))
        .unsafe_cast();

    IsEq::equal_under_law(lhs, rhs)
}
