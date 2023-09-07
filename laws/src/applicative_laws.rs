use rust2fun::prelude::*;

use crate::is_eq::IsEq;

pub fn applicative_identity<FA>(fa: FA) -> IsEq<FA>
where
    FA: Higher + Clone,
    FA::Target<fn(FA::Param) -> FA::Param>: Applicative + Apply<FA::Param, Target<FA::Param> = FA>,
{
    let lhs: FA = <FA::Target<fn(FA::Param) -> FA::Param>>::pure(id).ap(fa.clone());
    IsEq::equal_under_law(lhs, fa)
}

pub fn applicative_homomorphism<FA, FB, F>(a: FA::Param, mut f: F) -> IsEq<FB>
where
    FA: Applicative,
    FA::Param: Clone,
    FA::Target<F>:
        Applicative + Apply<FB::Param, Target<FB::Param> = FB> + Higher<Target<FA::Param> = FA>,
    FB: Applicative,
    F: FnMut(FA::Param) -> FB::Param,
{
    let lhs = Applicative::pure(f(a.clone()));
    let rhs = <FA::Target<F>>::pure(f).ap(Applicative::pure(a));
    IsEq::equal_under_law(lhs, rhs)
}

pub fn applicative_map<FA, B, F>(fa: FA, mut f: F) -> IsEq<FA::Target<B>>
where
    FA: Apply<B> + Clone,
    F: FnMut(FA::Param) -> B,
    FA::Target<F>:
        Applicative + Apply<B, Target<B> = FA::Target<B>> + Higher<Target<FA::Param> = FA>,
{
    let lhs = fa.clone().map(&mut f);
    let rhs = <FA::Target<F>>::pure(f).ap(fa);
    IsEq::equal_under_law(lhs, rhs)
}

pub fn ap_product_consistent<FA, B, F>(fa: FA, ff: FA::Target<F>) -> IsEq<FA::Target<B>>
where
    FA: Higher + Clone,
    F: Fn(FA::Param) -> B,
    FA::Target<F>: Apply<B, Target<B> = FA::Target<B>>
        + Higher<Target<FA::Param> = FA>
        + Semigroupal<FA::Param, Target<(F, FA::Param)> = FA::Target<(F, FA::Param)>>
        + Clone,
    FA::Target<(F, FA::Param)>: Functor<B>,
{
    let lhs = ff.clone().ap(fa.clone());
    let rhs = ff.product(fa).map(|(f, a)| f(a)).unsafe_cast();

    IsEq::equal_under_law(lhs, rhs)
}

pub fn applicative_unit<FA>(a: FA::Param) -> IsEq<FA>
where
    FA: Applicative,
    FA::Param: Clone,
    FA::Target<()>: Applicative + Functor<FA::Param, Target<FA::Param> = FA>,
{
    let lhs = <FA::Target<()>>::unit().map(|_| a.clone());
    let rhs = FA::pure(a);
    IsEq::equal_under_law(lhs, rhs)
}
