use std::str::FromStr;

use base64::prelude::*;
use kimchi::mina_curves::pasta::Fp;
use mina_p2p_messages::binprot::BinProtRead;
use mina_p2p_messages::v2::{MinaBaseProofStableV2, StateHash};
use mina_tree::proofs::verification::verify_block;
use mina_tree::proofs::verifier_index::{get_verifier_index, VerifierKind};
use mina_tree::verifier::get_srs;

// TODO(xqft): check proof size
const MAX_PROOF_SIZE: usize = 16 * 1024;
const MAX_PUB_INPUT_SIZE: usize = 1024;

#[no_mangle]
pub extern "C" fn verify_protocol_state_proof_ffi(
    proof_bytes: &[u8; MAX_PROOF_SIZE],
    proof_len: usize,
    public_input_bytes: &[u8; MAX_PUB_INPUT_SIZE],
    public_input_len: usize,
) -> bool {
    let protocol_state_proof_base64 =
        if let Ok(protocol_state_proof_base64) = std::str::from_utf8(&proof_bytes[..proof_len]) {
            protocol_state_proof_base64
        } else {
            return false;
        };
    let protocol_state_hash_base58 = if let Ok(protocol_state_hash_base58) =
        std::str::from_utf8(&public_input_bytes[..public_input_len])
    {
        protocol_state_hash_base58
    } else {
        return false;
    };

    let protocol_state_proof =
        if let Ok(protocol_state_proof) = parse_protocol_state_proof(protocol_state_proof_base64) {
            protocol_state_proof
        } else {
            return false;
        };
    let protocol_state_hash =
        if let Ok(protocol_state_hash) = parse_protocol_state_hash(protocol_state_hash_base58) {
            protocol_state_hash
        } else {
            return false;
        };

    let verifier_index = get_verifier_index(VerifierKind::Blockchain);
    let srs = get_srs::<Fp>();
    let srs = srs.lock().unwrap();

    verify_block(
        &protocol_state_proof,
        protocol_state_hash,
        &verifier_index,
        &srs,
    )
}

pub fn parse_protocol_state_proof(
    protocol_state_proof_base64: &str,
) -> Result<MinaBaseProofStableV2, String> {
    let protocol_state_proof_binprot = BASE64_URL_SAFE
        .decode(protocol_state_proof_base64.trim_end())
        .map_err(|err| err.to_string())?;

    MinaBaseProofStableV2::binprot_read(&mut protocol_state_proof_binprot.as_slice())
        .map_err(|err| err.to_string())
}

pub fn parse_protocol_state_hash(protocol_state_hash_base58: &str) -> Result<Fp, String> {
    StateHash::from_str(protocol_state_hash_base58.trim_end())
        .map_err(|err| err.to_string())?
        .to_fp()
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const PROTOCOL_STATE_PROOF_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state_proof.proof");
    const PROTOCOL_STATE_HASH_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state_hash.pub");
    const BAD_PROTOCOL_STATE_HASH_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/bad_protocol_state_hash.pub");

    const PROTOCOL_STATE_PROOF_STR: &str =
        include_str!("../../../../batcher/aligned/test_files/mina/protocol_state_proof.proof");
    const PROTOCOL_STATE_HASH_STR: &str =
        include_str!("../../../../batcher/aligned/test_files/mina/protocol_state_hash.pub");

    #[test]
    fn parse_protocol_state_proof_does_not_fail() {
        parse_protocol_state_proof(PROTOCOL_STATE_PROOF_STR).unwrap();
    }

    #[test]
    fn parse_protocol_state_hash_does_not_fail() {
        parse_protocol_state_hash(PROTOCOL_STATE_HASH_STR).unwrap();
    }

    #[test]
    fn protocol_state_proof_verifies() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROTOCOL_STATE_PROOF_BYTES.len();
        proof_buffer[..proof_size].clone_from_slice(PROTOCOL_STATE_PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = PROTOCOL_STATE_HASH_BYTES.len();
        pub_input_buffer[..pub_input_size].clone_from_slice(PROTOCOL_STATE_HASH_BYTES);

        let result = verify_protocol_state_proof_ffi(
            &proof_buffer,
            proof_size,
            &pub_input_buffer,
            pub_input_size,
        );
        assert!(result);
    }

    #[test]
    fn bad_protocol_state_proof_fails() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROTOCOL_STATE_PROOF_BYTES.len();
        proof_buffer[..proof_size].clone_from_slice(PROTOCOL_STATE_PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = BAD_PROTOCOL_STATE_HASH_BYTES.len();
        pub_input_buffer[..pub_input_size].clone_from_slice(BAD_PROTOCOL_STATE_HASH_BYTES);

        let result = verify_protocol_state_proof_ffi(
            &proof_buffer,
            proof_size,
            &pub_input_buffer,
            pub_input_size,
        );
        assert!(!result);
    }
}
