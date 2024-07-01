use ark_ff::BigInteger256;
use kimchi::{
    curve::KimchiCurve as _,
    mina_curves::pasta::{Fp, Fq, Vesta},
    mina_poseidon::sponge::DefaultFrSponge,
    plonk_sponge::FrSponge as _,
};

use super::state_proof::{ProofState, Statement};

pub fn tock_unpadded_public_input_of_statement(prev_statement: Statement) -> Vec<Fq> {
    let prev_statement_as_fields = vec![];

    let fp = [prev_statement.proof_state.deferred_values];

    prev_statement_as_fields
}
