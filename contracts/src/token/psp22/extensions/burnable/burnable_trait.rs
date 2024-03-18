// SPDX-License-Identifier: MIT

use super::super::PSP22Error;
use ink::{contract_ref, env::DefaultEnvironment, primitives::AccountId};
pub use pendzl::traits::Balance;
pub type PSP22BurnableRef = contract_ref!(PSP22Burnable, DefaultEnvironment);

/// trait extending PSP22 with burn functionality
#[ink::trait_definition]
pub trait PSP22Burnable {
    /// Destroys `amount` tokens from `account`, deducting from the caller's
    /// allowance.
    ///
    /// See [`PSP22Internal::_burn_from`].
    #[ink(message)]
    fn burn(
        &mut self,
        account: AccountId,
        amount: Balance,
    ) -> Result<(), PSP22Error>;
}
