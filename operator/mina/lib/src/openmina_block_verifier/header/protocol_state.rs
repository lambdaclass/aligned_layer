use std::str::FromStr as _;

use mina_p2p_messages::v2::{
    MinaStateProtocolStateBodyValueStableV2, MinaStateProtocolStateValueStableV2, StateHash,
};
use serde::Deserialize;

use super::blockchain_state::BlockchainState;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolState {
    pub previous_state_hash: String,
    pub blockchain_state: BlockchainState,
}

impl Into<MinaStateProtocolStateValueStableV2> for ProtocolState {
    fn into(self) -> MinaStateProtocolStateValueStableV2 {
        let previous_state_hash = StateHash::from_str(&self.previous_state_hash).unwrap();
        let body = MinaStateProtocolStateBodyValueStableV2 {
            genesis_state_hash: todo!(),
            blockchain_state: self.blockchain_state.into(),
            consensus_state: todo!(),
            constants: todo!(),
        };

        MinaStateProtocolStateValueStableV2 {
            previous_state_hash,
            body: todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use mina_p2p_messages::v2::StateHash;

    use crate::openmina_block_verifier::header::protocol_state_query::ProtocolStateQuery;

    const MINA_PROTOCOL_STATE_QUERY: &str = include_str!(
        "../../../../../../batcher/aligned/test_files/mina/mina_protocol_state_query.json"
    );

    #[test]
    fn test_protocol_state_parse() {
        let protocol_state_query: ProtocolStateQuery =
            serde_json::from_str(MINA_PROTOCOL_STATE_QUERY).unwrap();
        let protocol_state = &protocol_state_query.data.best_chain[0].protocol_state;
        let previous_state_hash = StateHash::from_str(&protocol_state.previous_state_hash).unwrap();
        assert_eq!(
            previous_state_hash.to_string(),
            protocol_state.previous_state_hash
        );
    }
}
