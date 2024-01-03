// SPDX-License-Identifier: MIT

/// Emitted when vested tokens are released
#[ink::event]
#[derive(Debug)]
pub struct TokenReleased {
    /// The locked asset.
    #[ink(topic)]
    pub asset: Option<AccountId>,
    /// The account to which the tokens are sent.
    #[ink(topic)]
    pub to: AccountId,
    /// The amount of tokens released.
    pub amount: Balance,
}
/// Emitted when vesting schedule is created
#[ink::event]
#[derive(Debug)]
pub struct VestingScheduled {
    /// The locked asset.
    #[ink(topic)]
    pub asset: Option<AccountId>,
    /// The account to which the tokens are sent.
    #[ink(topic)]
    pub to: AccountId,
    /// The amount of tokens released.
    pub amount: Balance,
    // The vesting start time.
    pub vesting_start: Timestamp,
    // The vesting end time.
    pub vesting_end: Timestamp,
}
