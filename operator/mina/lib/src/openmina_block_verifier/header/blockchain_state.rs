use mina_p2p_messages::{
    binprot::BinProtRead,
    v2::{
        ConsensusBodyReferenceStableV1, MinaBaseStagedLedgerHashStableV1,
        MinaStateBlockchainStateValueStableV2,
    },
};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainState {
    pub staged_ledger_hash: String,
    pub body_reference: String,
}

impl Into<MinaStateBlockchainStateValueStableV2> for BlockchainState {
    fn into(self) -> MinaStateBlockchainStateValueStableV2 {
        let staged_ledger_hash = parse_staged_ledger_hash(self.staged_ledger_hash).unwrap();
        // This value is in hex format
        let body_reference = ConsensusBodyReferenceStableV1(self.body_reference.as_str().into());
        MinaStateBlockchainStateValueStableV2 {
            staged_ledger_hash,
            genesis_ledger_hash: todo!(),
            ledger_proof_statement: todo!(),
            timestamp: todo!(),
            body_reference,
        }
    }
}

fn parse_staged_ledger_hash(
    input: String,
) -> Result<MinaBaseStagedLedgerHashStableV1, mina_p2p_messages::binprot::Error> {
    let mut decoded = &bs58::decode(input).into_vec().unwrap()[1..];
    MinaBaseStagedLedgerHashStableV1::binprot_read(&mut decoded)
}
