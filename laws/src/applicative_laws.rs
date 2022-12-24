use rust2fun::prelude::*;

use crate::is_eq::IsEq;

pub fn applicative_identity<FA>(fa: FA) -> IsEq<FA>
where
    FA: Apply<<FA as Higher>::Param, Target<<FA as Higher>::Param> = FA> + Clone + Eq,
    FA::Target<fn(FA::Param) -> FA::Param>: Applicative,
{
    let lhs = fa
        .clone()
        .ap::<fn(FA::Param) -> FA::Param>(<FA::Target<fn(FA::Param) -> FA::Param>>::pure(id));
    IsEq::equal_under_law(lhs, fa)
}

pub fn applicative_homomorphism<FA, FB, F>(a: FA::Param, mut f: F) -> IsEq<FB>
where
    FA: Applicative + Apply<FB::Param, Target<FB::Param> = FB>,
    FA::Param: Clone,
    FA::Target<F>: Applicative,
    FB: Applicative + Eq,
    F: FnMut(FA::Param) -> FB::Param,
{
    let lhs = FB::pure(f(a.clone()));
    let rhs = FA::pure(a).ap::<F>(<FA::Target<F>>::pure(f));
    IsEq::equal_under_law(lhs, rhs)
}

pub fn applicative_map<FA, B, F>(fa: FA, f: F) -> IsEq<FA::Target<B>>
where
    FA: Apply<B> + Clone,
    F: Fn(FA::Param) -> B + Clone,
    FA::Target<F>: Applicative,
    FA::Target<B>: Eq,
{
    let lhs = fa.clone().map(f.clone());
    let rhs = fa.ap::<F>(Applicative::pure(f));
    IsEq::equal_under_law(lhs, rhs)
}

pub fn ap_product_consistent<FA, B, F>(fa: FA, ff: FA::Target<F>) -> IsEq<FA::Target<B>>
where
    FA: Apply<B> + Semigroupal<F> + Clone,
    F: Fn(FA::Param) -> B,
    FA::Target<F>: Clone,
    FA::Target<(<FA as Higher>::Param, F)>: Functor<B>,
    FA::Target<B>: Eq,
{
    let lhs = fa.clone().ap(ff.clone());
    let rhs = fa.product(ff).map(|(a, f)| f(a)).unsafe_cast();
    IsEq::equal_under_law(lhs, rhs)
}

pub fn applicative_unit<FA>(a: FA::Param) -> IsEq<FA>
where
    FA: Applicative + Eq,
    FA::Param: Clone,
    FA::Target<()>: Applicative + Functor<FA::Param, Target<FA::Param> = FA>,
{
    let lhs = <FA::Target<()>>::unit().map(|_| a.clone());
    let rhs = FA::pure(a);
    IsEq::equal_under_law(lhs, rhs)
}
