use blake2::{Blake2b512, Digest};
use kimchi::o1_utils::FieldHelpers;
use mina_p2p_messages::{hash::MinaHash, v2::MinaStateProtocolStateValueStableV2};

#[derive(PartialEq)]
pub enum LongerChainResult {
    Tip,
    Candidate,
}

pub fn select_longer_chain(
    tip: &MinaStateProtocolStateValueStableV2,
    candidate: &MinaStateProtocolStateValueStableV2,
) -> LongerChainResult {
    let tip_block_height = &tip.body.consensus_state.blockchain_length.as_u32();
    let candidate_block_height = &candidate.body.consensus_state.blockchain_length.as_u32();

    if tip_block_height < candidate_block_height {
        return LongerChainResult::Candidate;
    }
    // tiebreak logic
    else if tip_block_height == candidate_block_height {
        // compare last VRF digests lexicographically
        if hash_last_vrf(tip) < hash_last_vrf(candidate) {
            return LongerChainResult::Candidate;
        } else if hash_last_vrf(tip) == hash_last_vrf(candidate) {
            // compare consensus state hashes lexicographically
            if hash_state(tip) < hash_state(candidate) {
                return LongerChainResult::Candidate;
            }
        }
    }

    LongerChainResult::Tip
}

fn hash_last_vrf(chain: &MinaStateProtocolStateValueStableV2) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(chain.body.consensus_state.last_vrf_output.as_slice());
    let digest = hasher.finalize().to_vec();

    String::from_utf8(digest).unwrap()
}

fn hash_state(chain: &MinaStateProtocolStateValueStableV2) -> String {
    MinaHash::hash(chain).to_hex()
}