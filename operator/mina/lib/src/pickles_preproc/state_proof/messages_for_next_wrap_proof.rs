use mina_tree::proofs::public_input::messages;
use serde::Deserialize;

use super::{
    utils::{bulletproof_challenges_array_to_fq_matrix, string_array_to_fq_inner_curve},
    BulletproofChallenge, Point,
};

#[derive(Deserialize)]
pub struct MessagesForNextWrapProof {
    pub challenge_polynomial_commitment: Point,
    pub old_bulletproof_challenges: [[BulletproofChallenge; 15]; 2],
}

impl Into<messages::MessagesForNextWrapProof> for MessagesForNextWrapProof {
    fn into(self) -> messages::MessagesForNextWrapProof {
        let challenge_polynomial_commitment =
            string_array_to_fq_inner_curve(&self.challenge_polynomial_commitment);
        let old_bulletproof_challenges =
            bulletproof_challenges_array_to_fq_matrix(&self.old_bulletproof_challenges);

        messages::MessagesForNextWrapProof {
            challenge_polynomial_commitment,
            old_bulletproof_challenges,
        }
    }
}
