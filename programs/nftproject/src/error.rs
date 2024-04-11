use anchor_lang::prelude::*;

#[error]

pub enum StakingError {
    #[msg("invalid User Pool")]
    InvalidUserPool,
    #[msg("Invalid Collection")]
    InvalidCollection,
    #[msg("Invalid User Pool")]
    InvalidAdmin,
    #[msg("Invalid Pool Number")]
    InvalidPoolError,
    #[msg(" NO Matching NFT to withdraw")]
    InvalidNFTAddress,
    #[msg("NFT OWNER Key mismatch")]
    InvalideOwner,
    #[msg("STaking Lock Now ")]
    InvalidWithdrawTIme,
    #[msg("withdraw NFT index overflow")]
    IndexOverflow,
    #[msg("YOu can't unstake before lockTime")]
    BeforeLockTime,
    #[msg("insufficient Lamport")]
    LackLamports,
    #[msg("can't Parse the NFT creator")]
    MetadataCreatorParseError,
    #[msg("invalid Metadata Address")]
    InvalidMetadata,

}