use ark_algebra_test_templates::{curves::*, groups::*};
use ark_ec::{boundary::serialize, AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{Field, One, PrimeField};
use ark_mnt4_298::{Fq4, Fr, MNT4_298};
use ark_native_boundary::boundary::CrossBoundary;
use ark_native_boundary::wrapped;
use ark_std::rand::Rng;
use ark_std::test_rng;
use ark_std::UniformRand;

type G1Projective = wrapped::G1Projective<MNT4_298>;
type G2Projective = wrapped::G2Projective<MNT4_298>;
type G1Affine = wrapped::G1Affine<MNT4_298>;
type G2Affine = wrapped::G2Affine<MNT4_298>;
type WrappedMNT4_298 = wrapped::EngineWrapper<MNT4_298>;

fn enable_fallback() {
    G1Projective::set_native_fallback(true);
    G1Affine::set_native_fallback(true);
}

#[test]
fn test_projective_noncanonical_serialization() {
    serialize::tests::test_serialize_projective::<G1Projective>();
    serialize::tests::test_serialize_projective::<G2Projective>();
}

#[test]
fn test_g1_projective_curve() {
    enable_fallback();
    curve_tests::<G1Projective>();

    // sw_tests::<g1::Parameters>();
}

#[test]
fn test_g1_projective_group() {
    let mut rng = test_rng();
    let a: G1Projective = rng.gen();
    let b: G1Projective = rng.gen();
    group_test(a, b);
}

#[test]
fn test_g1_generator() {
    let generator = G1Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_g2_projective_curve() {
    enable_fallback();
    curve_tests::<G2Projective>();

    // sw_tests::<g2::Parameters>();
}

#[test]
fn test_g2_projective_group() {
    let mut rng = test_rng();
    let a: G2Projective = rng.gen();
    let b: G2Projective = rng.gen();
    group_test(a, b);
}

#[test]
fn test_g2_generator() {
    let generator = G2Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_bilinearity() {
    let mut rng = test_rng();
    let a: G1Projective = rng.gen();
    let b: G2Projective = rng.gen();
    let s: Fr = rng.gen();

    let sa = a.mul(s.into_repr());
    let sb = b.mul(s.into_repr());

    let ans1 = WrappedMNT4_298::pairing(sa, b);
    let ans2 = WrappedMNT4_298::pairing(a, sb);
    let ans3 = WrappedMNT4_298::pairing(a, b).pow(s.into_repr());

    assert_eq!(ans1, ans2);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, Fq4::one());
    assert_ne!(ans2, Fq4::one());
    assert_ne!(ans3, Fq4::one());

    assert_eq!(ans1.pow(Fr::characteristic()), Fq4::one());
    assert_eq!(ans2.pow(Fr::characteristic()), Fq4::one());
    assert_eq!(ans3.pow(Fr::characteristic()), Fq4::one());
}

#[test]
fn test_product_of_pairings() {
    let rng = &mut test_rng();

    let a = G1Projective::rand(rng).into_affine();
    let b = G2Projective::rand(rng).into_affine();
    let c = G1Projective::rand(rng).into_affine();
    let d = G2Projective::rand(rng).into_affine();
    let ans1 = WrappedMNT4_298::pairing(a, b) * &WrappedMNT4_298::pairing(c, d);
    let ans2 = WrappedMNT4_298::product_of_pairings(&[(a.into(), b.into()), (c.into(), d.into())]);
    assert_eq!(ans1, ans2);
}
