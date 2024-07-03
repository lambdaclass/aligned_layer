use ark_ff::biginteger::BigInteger256;
use kimchi::mina_curves::pasta::{Fp, Pallas};
use mina_tree::proofs::{public_input::messages, transaction::InnerCurve};
use o1_utils::FieldHelpers;
use serde::Deserialize;
use std::array;

use super::{BulletproofChallenge, Point};

#[derive(Deserialize)]
pub struct MessagesForNextStepProof {
    pub challenge_polynomial_commitments: [Point; 2],
    pub old_bulletproof_challenges: [[BulletproofChallenge; 16]; 2],
}

impl<'a> Into<messages::MessagesForNextStepProof<'a, ()>> for MessagesForNextStepProof {
    fn into(self) -> messages::MessagesForNextStepProof<'a, ()> {
        messages::MessagesForNextStepProof {
            app_state: &(),
            // FIXME
            dlog_plonk_index: (),
            challenge_polynomial_commitments: string_arrays_to_inner_curve_vec(
                &self.challenge_polynomial_commitments,
            ),
            old_bulletproof_challenges: bulletproof_challenges_array_to_field_matrix(
                &self.old_bulletproof_challenges,
            ),
        }
    }
}

fn string_arrays_to_inner_curve_vec(input: &[[String; 2]; 2]) -> Vec<InnerCurve<Fp>> {
    input.iter().map(string_array_to_inner_curve).collect()
}

fn string_array_to_inner_curve(input: &[String; 2]) -> InnerCurve<Fp> {
    let x = Fp::from_hex(&input[0]).unwrap();
    let y = Fp::from_hex(&input[1]).unwrap();
    let affine = Pallas::new(x, y, false);

    InnerCurve::of_affine(affine)
}

fn bulletproof_challenges_array_to_field_matrix(
    input: &[[BulletproofChallenge; 16]; 2],
) -> Vec<[Fp; 16]> {
    input
        .iter()
        .map(bulletproof_challenge_array_to_field_vec)
        .collect()
}

fn bulletproof_challenge_array_to_field_vec(input: &[BulletproofChallenge; 16]) -> [Fp; 16] {
    array::from_fn(|i| {
        let inner = input[i].prechallenge.inner;
        let limbs = [inner[0] as u64, inner[1] as u64, 0, 0];
        Fp::new(BigInteger256::new(limbs))
    })
}
