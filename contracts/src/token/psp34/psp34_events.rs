// SPDX-License-Identifier: MIT

#[ink::event]
pub struct Transfer {
    from: Option<AccountId>,
    to: Option<AccountId>,
    id: Id,
}

#[ink::event]
pub struct Approval {
    from: AccountId,
    to: AccountId,
    id: Option<Id>,
    approved: bool,
}
