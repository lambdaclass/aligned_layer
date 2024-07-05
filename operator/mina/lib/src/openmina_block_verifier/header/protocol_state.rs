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
