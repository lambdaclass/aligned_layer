use mina_tree::proofs::{
    transaction::{StepProofState, StepStatement},
    unfinalized::Unfinalized,
};
use serde::Deserialize;

use super::{
    messages_for_next_step_proof::MessagesForNextStepProof,
    messages_for_next_wrap_proof::MessagesForNextWrapProof, DeferredValues,
};

#[derive(Deserialize)]
pub struct Statement {
    pub messages_for_next_step_proof: MessagesForNextStepProof,
    pub proof_state: ProofState,
}

#[derive(Deserialize)]
pub struct ProofState {
    pub deferred_values: DeferredValues,
    pub messages_for_next_wrap_proof: MessagesForNextWrapProof,
    pub sponge_digest_before_evaluations: [u64; 4],
}

impl Into<StepStatement> for Statement {
    fn into(self) -> StepStatement {
        let unfinalized = Unfinalized {
            deferred_values: self.proof_state.deferred_values.into(),
            // TODO: Check this bool
            should_finalize: false,
            sponge_digest_before_evaluations: self.proof_state.sponge_digest_before_evaluations,
        };

        let proof_state = StepProofState {
            unfinalized_proofs: vec![unfinalized],
            messages_for_next_step_proof: self.messages_for_next_step_proof.into(),
        };

        StepStatement {
            proof_state,
            messages_for_next_wrap_proof: vec![self
                .proof_state
                .messages_for_next_wrap_proof
                .into()],
        }
    }
}
