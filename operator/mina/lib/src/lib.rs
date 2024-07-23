mod consensus_state;

use ark_ec::short_weierstrass_jacobian::GroupAffine;
use base64::prelude::*;
use consensus_state::{select_longer_chain, LongerChainResult};
use kimchi::mina_curves::pasta::{Fp, PallasParameters};
use kimchi::o1_utils::FieldHelpers;
use kimchi::verifier_index::VerifierIndex;
use lazy_static::lazy_static;
use mina_p2p_messages::binprot::BinProtRead;
use mina_p2p_messages::hash::MinaHash;
use mina_p2p_messages::v2::{MinaBaseProofStableV2, MinaStateProtocolStateValueStableV2};
use mina_tree::proofs::verification::verify_block;
use mina_tree::verifier::get_srs;
use verifier_index::deserialize_blockchain_vk;

mod verifier_index;

lazy_static! {
    static ref VERIFIER_INDEX: VerifierIndex<GroupAffine<PallasParameters>> =
        deserialize_blockchain_vk().unwrap();
}

// TODO(xqft): check proof size
const MAX_PROOF_SIZE: usize = 16 * 1024;
const MAX_PUB_INPUT_SIZE: usize = 6 * 1024;
const PROTOCOL_STATE_HASH_SIZE: usize = 32;
// TODO(gabrielbosio): check that this length is always the same for every block
const PROTOCOL_STATE_SIZE: usize = 2060;

#[no_mangle]
pub extern "C" fn verify_protocol_state_proof_ffi(
    proof_bytes: &[u8; MAX_PROOF_SIZE],
    proof_len: usize,
    public_input_bytes: &[u8; MAX_PUB_INPUT_SIZE],
    public_input_len: usize,
) -> bool {
    let protocol_state_proof = match parse_protocol_state_proof(&proof_bytes[..proof_len]) {
        Ok(protocol_state_proof) => protocol_state_proof,
        Err(err) => {
            eprintln!("Failed to parse protocol state proof: {}", err);
            return false;
        }
    };

    let (
        candidate_protocol_state_hash,
        candidate_protocol_state,
        tip_protocol_state_hash,
        tip_protocol_state,
    ) = match parse_protocol_state_pub(&public_input_bytes[..public_input_len]) {
        Ok(protocol_state_pub) => protocol_state_pub,
        Err(err) => {
            eprintln!("Failed to parse protocol state public inputs: {}", err);
            return false;
        }
    };

    // TODO(xqft): this can be a batcher's pre-verification check (but don't remove it from here)
    if MinaHash::hash(&tip_protocol_state) != tip_protocol_state_hash {
        eprintln!("The tip's protocol state doesn't match the hash provided as public input");
        return false;
    }
    if MinaHash::hash(&candidate_protocol_state) != candidate_protocol_state_hash {
        eprintln!("The candidate's protocol state doesn't match the hash provided as public input");
        return false;
    }

    // TODO(xqft): srs should be a static, but can't make it so because it doesn't have all its
    // parameters initialized.
    let srs = get_srs::<Fp>();
    let srs = srs.lock().unwrap();

    if !verify_block(
        &protocol_state_proof,
        candidate_protocol_state_hash,
        &VERIFIER_INDEX,
        &srs,
    ) {
        return false;
    }

    // Consensus check: Short fork rule
    let longer_chain = select_longer_chain(&tip_protocol_state, &candidate_protocol_state);

    longer_chain == LongerChainResult::Candidate
}

pub fn parse_protocol_state_proof(
    protocol_state_proof_bytes: &[u8],
) -> Result<MinaBaseProofStableV2, String> {
    let protocol_state_proof_base64 =
        std::str::from_utf8(protocol_state_proof_bytes).map_err(|err| err.to_string())?;
    let protocol_state_proof_binprot = BASE64_URL_SAFE
        .decode(protocol_state_proof_base64)
        .map_err(|err| err.to_string())?;
    MinaBaseProofStableV2::binprot_read(&mut protocol_state_proof_binprot.as_slice())
        .map_err(|err| err.to_string())
}

pub fn parse_protocol_state_pub(
    protocol_state_pub: &[u8],
) -> Result<
    (
        Fp,
        MinaStateProtocolStateValueStableV2,
        Fp,
        MinaStateProtocolStateValueStableV2,
    ),
    String,
> {
    let (tip_protocol_state_hash, tip_protocol_state) = parse_protocol_state_with_hash(
        &protocol_state_pub[..(PROTOCOL_STATE_HASH_SIZE + PROTOCOL_STATE_SIZE)],
    )?;

    let (candidate_protocol_state_hash, candidate_protocol_state) = parse_protocol_state_with_hash(
        &protocol_state_pub[(PROTOCOL_STATE_HASH_SIZE + PROTOCOL_STATE_SIZE)
            ..((PROTOCOL_STATE_HASH_SIZE + PROTOCOL_STATE_SIZE) * 2)],
    )?;

    Ok((
        tip_protocol_state_hash,
        tip_protocol_state,
        candidate_protocol_state_hash,
        candidate_protocol_state,
    ))
}

fn parse_protocol_state_with_hash(
    protocol_state_pub: &[u8],
) -> Result<
    (
        ark_ff::Fp256<mina_curves::pasta::fields::FpParameters>,
        MinaStateProtocolStateValueStableV2,
    ),
    String,
> {
    let protocol_state_hash =
        Fp::from_bytes(&protocol_state_pub[..32]).map_err(|err| err.to_string())?;
    let protocol_state_base64 =
        std::str::from_utf8(&protocol_state_pub[32..]).map_err(|err| err.to_string())?;
    let protocol_state_binprot = BASE64_STANDARD
        .decode(protocol_state_base64)
        .map_err(|err| err.to_string())?;
    let protocol_state =
        MinaStateProtocolStateValueStableV2::binprot_read(&mut protocol_state_binprot.as_slice())
            .map_err(|err| err.to_string())?;

    Ok((protocol_state_hash, protocol_state))
}

#[cfg(test)]
mod test {
    use super::*;

    const PROTOCOL_STATE_PROOF_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state.proof");
    const PROTOCOL_STATE_PUB_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state.pub");
    const BAD_PROTOCOL_STATE_PUB_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/bad_protocol_state.pub");
    // BAD_PROTOCOL_STATE_PUB_BYTES has an invalid hash.

    #[test]
    fn parse_protocol_state_proof_does_not_fail() {
        parse_protocol_state_proof(PROTOCOL_STATE_PROOF_BYTES).unwrap();
    }

    #[test]
    fn parse_protocol_state_pub_does_not_fail() {
        parse_protocol_state_pub(PROTOCOL_STATE_PUB_BYTES).unwrap();
    }

    #[test]
    fn protocol_state_proof_verifies() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROTOCOL_STATE_PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROTOCOL_STATE_PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = PROTOCOL_STATE_PUB_BYTES.len();
        assert!(pub_input_size <= pub_input_buffer.len());
        pub_input_buffer[..pub_input_size].clone_from_slice(PROTOCOL_STATE_PUB_BYTES);

        let result = verify_protocol_state_proof_ffi(
            &proof_buffer,
            proof_size,
            &pub_input_buffer,
            pub_input_size,
        );
        assert!(result);
    }

    #[test]
    fn bad_protocol_state_proof_does_not_verify() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROTOCOL_STATE_PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROTOCOL_STATE_PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = BAD_PROTOCOL_STATE_PUB_BYTES.len();
        assert!(pub_input_size <= pub_input_buffer.len());
        pub_input_buffer[..pub_input_size].clone_from_slice(BAD_PROTOCOL_STATE_PUB_BYTES);

        let result = verify_protocol_state_proof_ffi(
            &proof_buffer,
            proof_size,
            &pub_input_buffer,
            pub_input_size,
        );
        assert!(!result);
    }
}
