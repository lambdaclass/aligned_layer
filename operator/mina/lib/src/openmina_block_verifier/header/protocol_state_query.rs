use serde::Deserialize;

use super::protocol_state::ProtocolState;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolStateQuery {
    pub data: Data,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub best_chain: [BestChain; 1],
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BestChain {
    pub protocol_state: ProtocolState,
}
