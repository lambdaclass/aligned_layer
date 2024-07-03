use std::rc::Rc;

use kimchi::mina_curves::pasta::Fp;
use mina_tree::proofs::transaction::{InnerCurve, ReducedMessagesForNextStepProof};
use serde::Deserialize;

use super::{
    utils::{bulletproof_challenges_array_to_fp_matrix, string_array_to_fp_inner_curve},
    BulletproofChallenge, Point,
};

#[derive(Deserialize)]
pub struct MessagesForNextStepProof {
    pub challenge_polynomial_commitments: [Point; 2],
    pub old_bulletproof_challenges: [[BulletproofChallenge; 16]; 2],
}

impl Into<ReducedMessagesForNextStepProof> for MessagesForNextStepProof {
    fn into(self) -> ReducedMessagesForNextStepProof {
        ReducedMessagesForNextStepProof {
            app_state: Rc::new(()),
            challenge_polynomial_commitments: string_arrays_to_inner_curve_vec(
                &self.challenge_polynomial_commitments,
            ),
            old_bulletproof_challenges: bulletproof_challenges_array_to_fp_matrix(
                &self.old_bulletproof_challenges,
            ),
        }
    }
}

fn string_arrays_to_inner_curve_vec(input: &[[String; 2]; 2]) -> Vec<InnerCurve<Fp>> {
    input.iter().map(string_array_to_fp_inner_curve).collect()
}
