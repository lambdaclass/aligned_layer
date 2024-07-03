use std::array;

use ark_ff::BigInteger256;
use kimchi::mina_curves::pasta::{Fp, Fq, Pallas, Vesta};
use mina_tree::proofs::transaction::InnerCurve;
use o1_utils::FieldHelpers as _;

use super::BulletproofChallenge;

pub fn string_array_to_fp_inner_curve(input: &[String; 2]) -> InnerCurve<Fp> {
    let x = Fp::from_hex(&input[0]).unwrap();
    let y = Fp::from_hex(&input[1]).unwrap();
    let affine = Pallas::new(x, y, false);

    InnerCurve::of_affine(affine)
}

pub fn string_array_to_fq_inner_curve(input: &[String; 2]) -> InnerCurve<Fq> {
    let x = Fq::from_hex(&input[0]).unwrap();
    let y = Fq::from_hex(&input[1]).unwrap();
    let affine = Vesta::new(x, y, false);

    InnerCurve::of_affine(affine)
}

pub fn bulletproof_challenges_array_to_fp_matrix(
    input: &[[BulletproofChallenge; 16]; 2],
) -> Vec<[Fp; 16]> {
    input
        .iter()
        .map(bulletproof_challenge_array_to_fp_vec)
        .collect()
}

pub fn bulletproof_challenge_array_to_fp_vec(input: &[BulletproofChallenge; 16]) -> [Fp; 16] {
    array::from_fn(|i| {
        let inner = input[i].prechallenge.inner;
        let limbs = [inner[0] as u64, inner[1] as u64, 0, 0];
        Fp::new(BigInteger256::new(limbs))
    })
}

pub fn bulletproof_challenges_array_to_fq_matrix(
    input: &[[BulletproofChallenge; 15]; 2],
) -> Vec<[Fq; 15]> {
    input
        .iter()
        .map(bulletproof_challenge_array_to_fq_vec)
        .collect()
}

pub fn bulletproof_challenge_array_to_fq_vec(input: &[BulletproofChallenge; 15]) -> [Fq; 15] {
    array::from_fn(|i| {
        let inner = input[i].prechallenge.inner;
        let limbs = [inner[0] as u64, inner[1] as u64, 0, 0];
        Fq::new(BigInteger256::new(limbs))
    })
}
