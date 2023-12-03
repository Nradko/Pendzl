// SPDX-License-Identifier: MIT

#[ink::event]
pub struct Paused {
    #[ink(topic)]
    account: AccountId,
}

#[ink::event]
pub struct Unpaused {
    #[ink(topic)]
    account: AccountId,
}
