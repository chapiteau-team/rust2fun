use rust2fun::compose;
use rust2fun::prelude::*;

use crate::is_eq::IsEq;

pub fn covariant_identity<FA>(fa: FA) -> IsEq<FA>
    where FA: Functor<<FA as Higher>::Param, Target<<FA as Higher>::Param>=FA> + Eq + Clone {
    IsEq::equal_under_law(fa.clone(), fa.map(id))
}

pub fn covariant_composition<FA, FB, FC>(
    fa: FA,
    mut f: impl FnMut(FA::Param) -> FB::Param,
    mut g: impl FnMut(FB::Param) -> FC::Param,
) -> IsEq<FC>
    where FA: Functor<FB::Param, Target<FB::Param>=FB> + Functor<FC::Param, Target<FC::Param>=FC> + Clone,
          FB: Functor<FC::Param, Target<FC::Param>=FC>,
          FC: Higher + Eq {
    IsEq::equal_under_law(
        fa.clone().map(&mut f).map(&mut g),
        fa.map(compose!(g, f)))
}

pub fn lift_identity<FA>(fa: FA) -> IsEq<FA>
    where FA: Functor<<FA as Higher>::Param, Target<<FA as Higher>::Param>=FA> + Eq + Clone {
    let f = lift(id);
    IsEq::equal_under_law(fa.clone(), f(fa))
}

pub fn lift_composition<FA, FB, FC>(
    fa: FA,
    mut f: impl FnMut(FA::Param) -> FB::Param,
    mut g: impl FnMut(FB::Param) -> FC::Param,
) -> IsEq<FC>
    where FA: Functor<FB::Param, Target<FB::Param>=FB> + Functor<FC::Param, Target<FC::Param>=FC> + Clone,
          FB: Functor<FC::Param, Target<FC::Param>=FC>,
          FC: Higher + Eq {
    let ff = lift(&mut f);
    let fg = lift(&mut g);
    let lhs = fg(ff(fa.clone()));
    let lgf = lift(compose!(g, f));
    let rhs = lgf(fa);
    IsEq::equal_under_law(lhs, rhs)
}