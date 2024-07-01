use ark_ff::BigInteger256;
use kimchi::{
    curve::KimchiCurve as _,
    mina_curves::pasta::{Fp, Vesta},
    mina_poseidon::sponge::DefaultFrSponge,
    plonk_sponge::FrSponge as _,
};
use o1_utils::FieldHelpers;

use super::state_proof::{FeatureFlags, Plonk, Point, Prechallenge, ProofState, Statement};

pub struct WrapDeferredValues {
    pub xi: Fp,
    pub plonk: DerivedPlonk,
}

pub struct DerivedPlonk {
    pub alpha: Prechallenge,
    pub beta: Point,
    pub feature_flags: FeatureFlags,
    pub gamma: Point,
    pub zeta: Prechallenge,
    pub zeta_to_srs_length: Fp,
    pub zeta_to_domain_size: Fp,
    perm: Fp,
}

pub fn expand_deferred(statement: Statement) -> WrapDeferredValues {
    let xi = squeeze_wrap_sponge(&statement.proof_state);
    let plonk = derive_plonk(&statement.proof_state.deferred_values.plonk);

    WrapDeferredValues { xi, plonk }
}

fn squeeze_wrap_sponge(proof_state: &ProofState) -> Fp {
    const CHALLENGE_CONSTANT_LENGTH: usize = 2;
    let mut sponge = DefaultFrSponge::new(Vesta::sponge_params());
    let digest = Fp::new(BigInteger256::new(
        proof_state.sponge_digest_before_evaluations,
    ));
    sponge.absorb(&digest);

    sponge.squeeze(CHALLENGE_CONSTANT_LENGTH)
}

fn derive_plonk(plonk: &Plonk) -> DerivedPlonk {
    DerivedPlonk {
        alpha: plonk.alpha,
        beta: plonk.beta,
        feature_flags: plonk.feature_flags,
        gamma: plonk.gamma,
        zeta: plonk.zeta,
    }
}

fn shift_value(value: Fp) -> Fp {
    // 2^255 + 1
    let two_to_size_plus_one =
        Fp::from_hex("0x3fffffffffffffffffffffffffffffffddb96703f6b306e466d2cf1300000000").unwrap();
    (value - two_to_size_plus_one) / Fp::from(2)
}
