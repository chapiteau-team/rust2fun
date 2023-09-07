mod common;

if_std! {
    use proptest::prelude::*;

    use rust2fun_laws::monoid_laws::*;
    use rust2fun_laws::semigroup_laws::*;

    proptest! {
        #[test]
        fn test_semigroup(fa: String, fb: String, fc: String) {
            assert!(repeat_0(fa.clone()).holds());
            assert!(repeat_1(fb.clone()).holds());
            assert!(semigroup_associativity(fa, fb, fc).holds());
        }

        #[test]
        fn test_monoid(fa: String) {
            assert!(monoid_left_identity(fa.clone()).holds());
            assert!(monoid_right_identity(fa.clone()).holds());
            assert!(is_id(fa).holds());
        }
    }
}
