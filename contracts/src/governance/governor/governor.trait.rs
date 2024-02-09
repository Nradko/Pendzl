#[ink::trait_definition]
pub trait Governor {
    #[ink(message)]
    fn hash_description(&self, description: String) -> Result<Hash, GovernanceError>;

    /// Hashing function used to (re)build the proposal id from the proposal details.
    #[ink(message)]
    fn hash_proposal(&self, proposal: Proposal) -> Result<ProposalHash, GovernanceError>;

    /// Current status of a proposal.
    #[ink(message)]
    fn status(&self, proposal_id: ProposalId) -> Result<ProposalStatus, GovernanceError>;

    ///
    #[ink(message)]
    fn proposal(&self, proposal_id: ProposalId) -> Result<Proposal, GovernanceError>;

    /// Returns the number of votes already casted for a proposal by a given account
    #[ink(message)]
    fn get_votes_with_params(
        &mut self,
        account: AccountId,
        timestamp: Timestamp,
        params: Vec<u8>,
    ) -> Result<u128, GovernanceError>;

    /// Makes a proposal for a list of transactions to be executed.
    /// Returns the id of the proposal
    #[ink(message)]
    fn propose(
        &mut self,
        transactions: Vec<Transaction>,
        description: String,
    ) -> Result<ProposalId, GovernanceError>;

    #[ink(message)]
    fn finalize(&mut self, proposal_id: ProposalId) -> Result<(), GovernanceError>;

    /// Executes a proposal if it is in the `Succeeded` state.
    /// Returns the id of the executed proposal
    #[ink(message)]
    fn execute(&mut self, proposal: Proposal) -> Result<(), GovernanceError>;

    /// Cancels a proposal if it is in the `Pending` state.
    /// Returns the id of the cancelled proposal
    #[ink(message)]
    fn cancel(&mut self, proposal_id: ProposalId) -> Result<(), GovernanceError>;

    /// Casts a vote for a proposal from a message sender.
    /// Returns the number of votes already casted for the proposal by the sender
    #[ink(message)]
    fn cast_vote(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: Option<String>,
        params: Option<Vec<u8>>,
    ) -> Result<Balance, GovernanceError>;

    /// Relays a transaction or function call to an arbitrary target. In cases where the governance executor
    /// is some contract other than the governor itself, like when using a timelock, this function can be invoked
    /// in a governance proposal to recover tokens or Ether that was sent to the governor contract by mistake.
    #[ink(message)]
    fn relay(&mut self, target: AccountId, transaction: Transaction)
        -> Result<(), GovernanceError>;
}

pub trait GovernorInternal {
    fn _hash_description(&self, description: String) -> Result<Hash, GovernanceError>;
    fn _hash_proposal(&self, proposal: Proposal) -> Result<ProposalHash, GovernanceError>;

    fn _get_votes_with_params(
        &mut self,
        account: AccountId,
        timestamp: Timestamp,
        params: Vec<u8>,
    ) -> Result<u128, GovernanceError>;

    fn _cast_vote(
        &mut self,
        proposal_id: ProposalId,
        support: VoteType,
        reason: Option<String>,
        params: Option<Vec<u8>>,
    ) -> Result<Balance, GovernanceError>;

    fn _eceute(&mut self, proposal: Proposal) -> Result<ProposalId, GovernanceError>;

    fn _propose(
        &mut self,
        transactions: Vec<Transaction>,
        description: String,
    ) -> Result<ProposalId, GovernanceError>;

    fn _cancel(&mut self, proposal_id: ProposalId) -> Result<ProposalId, GovernanceError>;
}
