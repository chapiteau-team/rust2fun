use rust2fun::prelude::*;

use crate::is_eq::IsEq;

pub fn monoid_left_identity<A>(a: A) -> IsEq<A>
where
    A: Monoid + Clone,
{
    IsEq::equal_under_law(a.clone(), A::empty().combine(a))
}

pub fn monoid_right_identity<A>(a: A) -> IsEq<A>
where
    A: Monoid + Clone,
{
    IsEq::equal_under_law(a.clone(), a.combine(A::empty()))
}

pub fn is_id<A>(a: A) -> IsEq<bool>
where
    A: Monoid + Clone + Eq,
{
    IsEq::equal_under_law(a.clone() == A::empty(), a.is_empty())
}
