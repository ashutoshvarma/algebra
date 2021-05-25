use ark_ed_on_bls12_377::{EdwardsAffine as EdAffine, EdwardsProjective as EdProjective};

use ark_algebra_test_templates::{curves::*, groups::*};
use ark_ec::models::wrapped::{GroupAffine, GroupProjective};
use ark_ec::wrapped;
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_std::rand::Rng;
use ark_std::test_rng;

type EdwardsProjective = GroupProjective<EdProjective>;
type EdwardsAffine = GroupAffine<EdAffine>;

#[test]
fn test_projective_noncanonical_serialization() {
    wrapped::serialize::tests::test_serialize_projective::<EdwardsProjective>();
}

#[test]
fn test_projective_curve() {
    curve_tests::<EdwardsProjective>();

    // edwards_tests::<EdwardsParameters>();
}

#[test]
fn test_projective_group() {
    let mut rng = test_rng();
    let a = rng.gen();
    let b = rng.gen();
    for _i in 0..100 {
        group_test::<EdwardsProjective>(a, b);
    }
}

#[test]
fn test_affine_group() {
    let mut rng = test_rng();
    let a: EdwardsAffine = rng.gen();
    let b: EdwardsAffine = rng.gen();
    for _i in 0..100 {
        group_test::<EdwardsAffine>(a, b);
    }
}

#[test]
fn test_generator() {
    let generator = EdwardsAffine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_conversion() {
    let mut rng = test_rng();
    let a: EdwardsAffine = rng.gen();
    let b: EdwardsAffine = rng.gen();
    let a_b = {
        use ark_ec::group::Group;
        (a + &b).double().double()
    };
    let a_b2 = (a.into_projective() + &b.into_projective())
        .double()
        .double();
    assert_eq!(a_b, a_b2.into_affine());
    assert_eq!(a_b.into_projective(), a_b2);
}

// #[test]
// fn test_montgomery_conversion() {
//     montgomery_conversion_test::<EdwardsParameters>();
// }
