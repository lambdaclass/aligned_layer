use mina_tree::proofs::transaction::StepStatement;
use serde::Deserialize;

use super::{messages_for_next_step_proof::MessagesForNextStepProof, ProofState};

#[derive(Deserialize)]
pub struct Statement {
    pub messages_for_next_step_proof: MessagesForNextStepProof,
    pub proof_state: ProofState,
}

impl Into<StepStatement> for Statement {
    fn into(self) -> StepStatement {
        StepStatement {
            proof_state: self.proof_state.into(),
            messages_for_next_wrap_proof: vec![self
                .proof_state
                .messages_for_next_wrap_proof
                .into()],
        }
    }
}
