use rust2fun::compose;
use rust2fun::prelude::*;

use crate::is_eq::IsEq;

pub fn invariant_identity<FA>(fa: FA) -> IsEq<FA>
    where FA: Invariant<<FA as Higher>::Param, Target<<FA as Higher>::Param>=FA> + Eq + Clone {
    IsEq::equal_under_law(fa.clone(), fa.imap(id, id))
}

pub fn invariant_composition<FA, FB, FC>(
    fa: FA,
    mut f1: impl FnMut(FA::Param) -> FB::Param,
    mut f2: impl FnMut(FB::Param) -> FA::Param,
    mut g1: impl FnMut(FB::Param) -> FC::Param,
    mut g2: impl FnMut(FC::Param) -> FB::Param,
) -> IsEq<FC>
    where FA: Invariant<FB::Param, Target<FB::Param>=FB> + Invariant<FC::Param, Target<FC::Param>=FC> + Clone,
          FB: Invariant<FC::Param, Target<FC::Param>=FC>,
          FC: Higher + Eq {
    IsEq::equal_under_law(
        fa.clone().imap(&mut f1, &mut f2).imap(&mut g1, &mut g2),
        fa.imap(compose!(g1, f1), compose!(f2, g2)))
}