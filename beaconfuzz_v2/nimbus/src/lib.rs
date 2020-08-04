use types::{
    Attestation, AttesterSlashing, BeaconState, Deposit, MainnetEthSpec, ProposerSlashing,
    SignedBeaconBlock, SignedVoluntaryExit,
};

pub mod attestation;
pub fn process_attestation(
    beacon: &BeaconState<MainnetEthSpec>,
    attest: &Attestation<MainnetEthSpec>,
    post: &[u8],
) -> bool {
    self::attestation::process_attestation(beacon, attest, post)
}

pub mod attester_slashing;
pub fn process_attester_slashing(
    beacon: &BeaconState<MainnetEthSpec>,
    attester_slashing: &AttesterSlashing<MainnetEthSpec>,
    post: &[u8],
) -> bool {
    self::attester_slashing::process_attester_slashing(beacon, attester_slashing, post)
}

pub mod block;
pub fn process_block(
    beacon: &BeaconState<MainnetEthSpec>,
    beacon_block: &SignedBeaconBlock<MainnetEthSpec>,
    post: &[u8],
) -> bool {
    self::block::process_block(beacon, beacon_block, post)
}

pub mod block_header;
pub fn process_block_header(
    beacon: &BeaconState<MainnetEthSpec>,
    beacon_block: &SignedBeaconBlock<MainnetEthSpec>,
    post: &[u8],
) -> bool {
    self::block_header::process_block_header(beacon, beacon_block, post)
}

pub mod deposit;
pub fn process_deposit(
    beacon: &BeaconState<MainnetEthSpec>,
    deposit: &Deposit,
    post: &[u8],
) -> bool {
    self::deposit::process_deposit(beacon, deposit, post)
}

pub mod proposer_slashing;
pub fn process_proposer_slashing(
    beacon: &BeaconState<MainnetEthSpec>,
    proposer_slashing: &ProposerSlashing,
    post: &[u8],
) -> bool {
    self::proposer_slashing::process_proposer_slashing(beacon, proposer_slashing, post)
}

pub mod voluntary_exit;
pub fn process_voluntary_exit(
    beacon: &BeaconState<MainnetEthSpec>,
    exit: &SignedVoluntaryExit,
    post: &[u8],
) -> bool {
    self::voluntary_exit::process_voluntary_exit(beacon, exit, post)
}
