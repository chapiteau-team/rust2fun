extern crate rust2fun_laws;

use rust2fun_laws::applicative_laws::*;
use rust2fun_laws::apply_laws::*;
use rust2fun_laws::flatmap_laws::*;
use rust2fun_laws::functor_laws::*;
use rust2fun_laws::invariant_laws::*;
use rust2fun_laws::semigroupal_laws::*;

use crate::common::{parse, print};

mod common;

#[test]
fn test_invariant() {
    assert!(invariant_identity(Err::<(), _>(true)).holds());
    assert!(invariant_identity(Ok::<_, bool>(1)).holds());

    let invariant_composition_for = |x| invariant_composition(x, print, parse, parse::<i32>, print);
    assert!(invariant_composition_for(Err(true)).holds());
    assert!(invariant_composition_for(Ok(1)).holds());
}

#[test]
fn test_functor() {
    assert!(covariant_identity(Err::<(), _>(true)).holds());
    assert!(covariant_identity(Ok::<_, bool>(1)).holds());

    let covariant_composition_for = |x| covariant_composition(x, print, parse::<i64>);
    assert!(covariant_composition_for(Err(true)).holds());
    assert!(covariant_composition_for(Ok(1)).holds());

    assert!(lift_identity(Err::<(), _>(true)).holds());
    assert!(lift_identity(Ok::<_, bool>(1)).holds());

    let lift_composition_for = |x| lift_composition(x, print, parse::<u8>);
    assert!(lift_composition_for(Err(true)).holds());
    assert!(lift_composition_for(Ok(1)).holds());
}

#[test]
fn test_semigroupal() {
    assert!(semigroupal_associativity(Ok::<_, ()>(1), Ok("ok".to_string()), Ok(true)).holds());
    assert!(semigroupal_associativity(
        Ok::<_, bool>(1),
        Ok("ok".to_string()),
        Err::<Option<()>, _>(false)
    )
    .holds());
}

#[test]
fn test_apply() {
    let check_length = |x: &str, l: usize| x.len() == l;

    assert!(map2_product_consistency(Ok::<_, ()>("str"), Ok(1), check_length).holds());
    assert!(map2_product_consistency(Ok("str"), Err(()), check_length).holds());
    assert!(map2_product_consistency(Err(()), Ok(1), check_length).holds());
    assert!(map2_product_consistency(Err(()), Err(()), check_length).holds());

    assert!(product_r_consistency(Ok::<_, ()>("str"), Ok(1)).holds());
    assert!(product_r_consistency(Ok("str"), Err::<i32, _>(())).holds());
    assert!(product_r_consistency(Err::<i32, _>(()), Ok(1)).holds());
    assert!(product_r_consistency(Err::<i32, _>(()), Err::<i32, _>(())).holds());

    assert!(product_l_consistency(Ok::<_, ()>("str"), Ok(1)).holds());
    assert!(product_l_consistency(Ok("str"), Err::<i32, _>(())).holds());
    assert!(product_l_consistency(Err::<i32, _>(()), Ok(1)).holds());
    assert!(product_l_consistency(Err::<i32, _>(()), Err::<i32, _>(())).holds());
}

#[test]
fn test_applicative() {
    assert!(applicative_identity(Ok::<_, ()>(1)).holds());
    assert!(applicative_identity(Err::<i32, _>(false)).holds());

    assert!(applicative_homomorphism::<Result<_, ()>, _, _>(1, print).holds());

    assert!(applicative_map(Ok::<_, ()>(1), print).holds());
    assert!(applicative_map(Err(()), print::<i32>).holds());

    assert!(ap_product_consistent(Err(false), Ok(print::<i32>)).holds());
    assert!(ap_product_consistent(Ok::<_, bool>(1), Ok(print)).holds());

    assert!(applicative_unit::<Result<_, ()>>(1).holds());
}

#[test]
fn test_flatmap() {
    assert!(
        flat_map_associativity(Ok::<_, ()>(1), |x| Ok(print(x)), |s| Ok(parse::<i32>(s))).holds()
    );
    assert!(flat_map_associativity(
        Err::<u32, _>(false),
        |x| Ok(print(x)),
        |s| Ok(parse::<i32>(s))
    )
    .holds());

    assert!(flat_map_consistent_apply(Ok::<_, ()>(1), Ok(print)).holds());
    assert!(flat_map_consistent_apply(Err::<u32, _>(false), Ok(print)).holds());

    assert!(m_product_consistency(Ok::<_, ()>(1), |x| Ok(print(x))).holds());
    assert!(m_product_consistency(Err::<u32, _>(false), |x| Ok(print(x))).holds());
}
