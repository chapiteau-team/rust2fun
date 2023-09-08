use rust2fun::prelude::*;

use crate::is_eq::IsEq;

pub fn bifunctor_identity<FAB>(fab: FAB) -> IsEq<FAB>
where
    FAB: Bifunctor<
            <FAB as Higher2>::Param1,
            <FAB as Higher2>::Param2,
            Target<<FAB as Higher2>::Param1, <FAB as Higher2>::Param2> = FAB,
        > + Clone,
{
    IsEq::equal_under_law(fab.clone(), fab.bimap(id, id))
}

pub fn bifunctor_composition<FAX, FBY, FCZ>(
    fax: FAX,
    mut f1: impl FnMut(FAX::Param1) -> FBY::Param1,
    mut f2: impl FnMut(FBY::Param1) -> FCZ::Param1,
    mut g1: impl FnMut(FAX::Param2) -> FBY::Param2,
    mut g2: impl FnMut(FBY::Param2) -> FCZ::Param2,
) -> IsEq<FCZ>
where
    FAX: Bifunctor<FBY::Param1, FBY::Param2, Target<FBY::Param1, FBY::Param2> = FBY>
        + Bifunctor<FCZ::Param1, FCZ::Param2, Target<FCZ::Param1, FCZ::Param2> = FCZ>
        + Clone,
    FBY: Bifunctor<FCZ::Param1, FCZ::Param2, Target<FCZ::Param1, FCZ::Param2> = FCZ>,
    FCZ: Higher2,
{
    IsEq::equal_under_law(
        fax.clone().bimap(&mut f1, &mut g1).bimap(&mut f2, &mut g2),
        fax.bimap(compose!(f2, f1), compose!(g2, g1)),
    )
}
