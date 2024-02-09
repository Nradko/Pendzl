pub type ProposalId = u32;
pub type ProposalHash = Hash;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Proposal {
    pub transactions: Vec<Transaction>,
    pub description_hash: Hash,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Transaction {
    pub callee: Option<AccountId>,
    pub selector: [u8; 4],
    pub input: Vec<u8>,
    pub transferred_value: Balance,
    pub gas_limit: u64,
}

#[derive(scale::Decode, scale::Encode, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct ProposalCore {
    pub proposer: AccountId,
    pub vote_start: Timestamp,
    pub vote_duration: Timestamp,
    pub status: ProposalStatus,
}

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum ProposalStatus {
    #[default]
    Pending,
    Active,
    Canceled,
    Defeated,
    Succeeded,
    Queued,
    Expired,
    Executed,
}

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum VoteType {
    #[default]
    Against = 1,
    For = 2,
}
