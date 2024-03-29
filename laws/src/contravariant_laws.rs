use rust2fun::prelude::*;

use crate::is_eq::IsEq;

pub fn contravariant_identity<FA>(fa: FA) -> IsEq<FA>
where
    FA: Contravariant<<FA as Higher>::Param, Target<<FA as Higher>::Param> = FA> + Clone,
{
    IsEq::equal_under_law(fa.clone(), fa.contramap(id))
}

pub fn contravariant_composition<FA, FB, FC>(
    fa: FA,
    mut f: impl FnMut(FB::Param) -> FA::Param,
    mut g: impl FnMut(FC::Param) -> FB::Param,
) -> IsEq<FC>
where
    FA: Contravariant<FB::Param, Target<FB::Param> = FB>
        + Contravariant<FC::Param, Target<FC::Param> = FC>
        + Clone,
    FB: Contravariant<FC::Param, Target<FC::Param> = FC>,
    FC: Higher,
{
    IsEq::equal_under_law(
        fa.clone().contramap(&mut f).contramap(&mut g),
        fa.contramap(compose!(f, g)),
    )
}

pub fn lift_contravariant_identity<FA>(fa: FA) -> IsEq<FA>
where
    FA: Contravariant<<FA as Higher>::Param, Target<<FA as Higher>::Param> = FA> + Clone,
{
    let mut f = lift_contravariant(id);
    IsEq::equal_under_law(fa.clone(), f(fa))
}

pub fn lift_contravariant_composition<FA, FB, FC>(
    fa: FA,
    mut f: impl FnMut(FB::Param) -> FA::Param,
    mut g: impl FnMut(FC::Param) -> FB::Param,
) -> IsEq<FC>
where
    FA: Contravariant<FB::Param, Target<FB::Param> = FB>
        + Contravariant<FC::Param, Target<FC::Param> = FC>
        + Clone,
    FB: Contravariant<FC::Param, Target<FC::Param> = FC>,
    FC: Higher,
{
    let lhs = {
        let mut ff = lift_contravariant(&mut f);
        let mut fg = lift_contravariant(&mut g);
        fg(ff(fa.clone()))
    };

    let mut lgf = lift_contravariant(compose!(f, g));
    let rhs = lgf(fa);
    IsEq::equal_under_law(lhs, rhs)
}
