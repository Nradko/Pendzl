use pendzl::math::errors::MathError;
use pendzl::traits::String;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum GovernanceError {
    TokenNotSet,
    InvalidQuorumFraction,
    AlreadyCastVote,
    DisabledDeposit,
    OnlyProposer,
    OnlyExecutor,
    NonexistentProposal,
    UnexpectedProposalState,
    InvalidVotingPeriod,
    InsufficientProposerVotes,
    InvalidVoteType,
    InvalidSignature,
    ProposerRestricted,
    InvalidDestination,
    ZeroSnapshot,
    DeadlineOverflow,
    ZeroProposalLength,
    ProposalNotFound,
    InvalidInput,
    UnderlyingTransactionReverted,
    ProposalAlreadyExists,
    ErrorParsingDescription,
    FutureLookup,
    ExpiredSignature,
    ExecutionFailed,
    IndexOutOfRange,
    Custom(String),
}

impl From<MathError> for GovernanceError {
    fn from(err: MathError) -> Self {
        match err {
            MathError::Overflow => GovernanceError::Custom(String::from("M::Overflow")),
            MathError::Underflow => GovernanceError::Custom(String::from("M::Underflow")),
            MathError::DivByZero => GovernanceError::Custom(String::from("M::DivByZero")),
        }
    }
}
