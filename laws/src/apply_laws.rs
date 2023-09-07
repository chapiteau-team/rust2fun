use rust2fun::prelude::*;

use crate::is_eq::IsEq;

pub fn map2_product_consistency<FA, FB, FC, F>(fa: FA, fb: FB, mut f: F) -> IsEq<FC>
where
    FA: Apply<FB::Param> + Higher<Target<FB::Param> = FB> + Higher<Target<FC::Param> = FC> + Clone,
    FB: Higher + Clone,
    FC: Higher,
    F: FnMut(FA::Param, FB::Param) -> FC::Param,
    FA::Target<(FA::Param, FB::Param)>: Functor<FC::Param, Target<FC::Param> = FC>,
{
    let lhs = fa.clone().product(fb.clone()).map(|(a, b)| f(a, b));
    let rhs = fa.map2(fb, f);

    IsEq::equal_under_law(lhs, rhs)
}

pub fn product_r_consistency<FA, FB>(fa: FA, fb: FB) -> IsEq<FB>
where
    FA: Apply<FB::Param> + Higher<Target<FB::Param> = FB> + Clone,
    FB: Higher + Clone,
    FA::Target<(FA::Param, FB::Param)>: Functor<FB::Param, Target<FB::Param> = FB>,
{
    let lhs = fa.clone().product_r(fb.clone());
    let rhs = fa.map2(fb, |_, b| b);

    IsEq::equal_under_law(lhs, rhs)
}

pub fn product_l_consistency<FA, FB>(fa: FA, fb: FB) -> IsEq<FA>
where
    FA: Apply<FB::Param>
        + Higher<Target<FB::Param> = FB>
        + Higher<Target<<FA as Higher>::Param> = FA>
        + Clone,
    FB: Higher + Clone,
    FA::Target<(FA::Param, FB::Param)>: Functor<FA::Param, Target<FA::Param> = FA>,
{
    let lhs = fa.clone().product_l(fb.clone());
    let rhs = fa.map2(fb, |a, _| a);

    IsEq::equal_under_law(lhs, rhs)
}
