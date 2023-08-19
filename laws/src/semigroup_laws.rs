use std::fmt::Debug;

use rust2fun::prelude::*;

use crate::is_eq::IsEq;

pub fn semigroup_associativity<A: Debug>(a: A, b: A, c: A) -> IsEq<A>
where
    A: Semigroup + Clone + Eq,
{
    let lhs = a.clone().combine(b.clone()).combine(c.clone());
    let rhs = a.combine(b.combine(c));

    IsEq::equal_under_law(lhs, rhs)
}

pub fn repeat_0<A>(a: A) -> IsEq<A>
where
    A: Semigroup + Clone + Eq,
{
    IsEq::equal_under_law(a.clone(), a.combine_n(0))
}

pub fn repeat_1<A>(a: A) -> IsEq<A>
where
    A: Semigroup + Clone + Eq,
{
    IsEq::equal_under_law(a.clone().combine(a.clone()), a.combine_n(1))
}
