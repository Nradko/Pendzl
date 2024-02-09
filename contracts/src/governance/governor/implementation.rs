pub use ink::primitives::{AccountId, Hash};
pub use ink::storage::Mapping;
use pendzl::traits::Balance;

use super::{GovernanceError, Proposal};
pub use super::{ProposalCore, ProposalHash, ProposalId, VoteType};

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct GovernorData<VoteTypes: scale::Decode + scale::Encode> {
    id_to_hash: Mapping<ProposalId, Hash>,
    hash_to_id: Mapping<ProposalHash, ProposalId>,
    proposals: Mapping<ProposalId, ProposalCore>,
    proposal_votes: Mapping<(ProposalId, VoteTypes), u128>,
    has_voted_for: Mapping<(AccountId, ProposalId), VoteTypes>,
}

pub trait Finalization {
    fn is_finalized(&self, proposal_id: ProposalId) -> Result<bool, GovernanceError>;
}

pub trait GovernorStorage {
    fn create_proposal(&mut self, proposal: ProposalCore) -> Result<ProposalId, GovernanceError>;

    fn cancel_proposal(&mut self, proposal_id: ProposalId) -> Result<ProposalId, GovernanceError>;

    fn finalize_proposal(&mut self, proposal_id: ProposalId)
        -> Result<ProposalId, GovernanceError>;

    fn execute_proposal(&mut self, proposal: Proposal) -> Result<ProposalId, GovernanceError>;

    fn cast_vote(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: Option<String>,
        params: Option<Vec<u8>>,
    ) -> Result<Balance, GovernanceError>;
}

impl<VoteTypes: scale::Encode + scale::Decode> GovernorStorage for GovernorData<VoteTypes> where
    GovernorData<VoteTypes>: Finalization
{
}
