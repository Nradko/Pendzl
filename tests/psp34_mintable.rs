// SPDX-License-Identifier: MIT
// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#[cfg(feature = "psp34")]
#[pendzl::implementation(PSP34, PSP34Mintable)]
#[ink::contract]
mod psp34_mintable {
    use pendzl::{
        contracts::token::psp34::{Id, PSP34Error, PSP34},
        test_utils::accounts,
        traits::String,
    };

    #[derive(Default, StorageFieldGetter)]
    #[ink(storage)]
    pub struct PSP34Struct {
        #[storage_field]
        psp34: PSP34Data,
        // field for testing _before_token_transfer
        return_err_on_before: bool,
        // field for testing _after_token_transfer
        return_err_on_after: bool,
    }

    #[overrider(PSP34Internal)]
    fn _update(
        &mut self,
        from: &Option<&AccountId>,
        to: &Option<&AccountId>,
        id: &Id,
    ) -> Result<(), PSP22Error> {
        if self.return_err_on_before {
            return Err(PSP34Error::Custom(String::from(
                "Error on _before_token_transfer",
            )));
        }
        pendzl::contracts::token::psp34::implementation::PSP34InternalDefaultImpl::_update_default_impl(
            self, from, to, id,
        )?;

        if self.return_err_on_after {
            return Err(PSP34Error::Custom(String::from(
                "Error on _after_token_transfer",
            )));
        }
        Ok(())
    }

    impl PSP34Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        pub fn change_state_err_on_before(&mut self) {
            self.return_err_on_before = !self.return_err_on_before;
        }

        pub fn change_state_err_on_after(&mut self) {
            self.return_err_on_after = !self.return_err_on_after;
        }
    }

    #[ink::test]
    fn mint_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Token 1 does not _exists.
        assert_eq!(PSP34::owner_of(&mut nft, Id::U8(1u8)), None);
        // Alice does not owns tokens.
        assert_eq!(PSP34::balance_of(&mut nft, accounts.alice), 0);
        // Create token Id 1.
        assert!(
            PSP34Mintable::mint(&mut nft, accounts.alice, Id::U8(1u8)).is_ok()
        );
        // Alice owns 1 token.
        assert_eq!(PSP34::balance_of(&mut nft, accounts.alice), 1);
    }

    #[ink::test]
    fn mint_existing_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Create token Id 1.
        assert!(
            PSP34Mintable::mint(&mut nft, accounts.alice, Id::U8(1u8)).is_ok()
        );
        // Alice owns 1 token.
        assert_eq!(PSP34::balance_of(&mut nft, accounts.alice), 1);
        // Alice owns token Id 1.
        assert_eq!(
            PSP34::owner_of(&mut nft, Id::U8(1u8)),
            Some(accounts.alice)
        );
        // Cannot create token Id if it _exists.
        assert_eq!(
            PSP34Mintable::mint(&mut nft, accounts.alice, Id::U8(1u8)),
            Err(PSP34Error::TokenExists)
        );
        assert_eq!(
            PSP34Mintable::mint(&mut nft, accounts.bob, Id::U8(1u8)),
            Err(PSP34Error::TokenExists)
        );
    }

    #[ink::test]
    fn before_token_transfer_should_fail_mint() {
        // Constructor works.
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Can mint token to Alice
        assert!(
            PSP34Mintable::mint(&mut nft, accounts.alice, Id::U8(1u8)).is_ok()
        );
        assert_eq!(PSP34::balance_of(&mut nft, accounts.alice), 1);
        // Turn on error on _before_token_transfer
        nft.change_state_err_on_before();
        // Alice gets an error on _before_token_transfer
        assert_eq!(
            PSP34Mintable::mint(&mut nft, accounts.alice, Id::U8(4u8)),
            Err(PSP34Error::Custom(String::from(
                "Error on _before_token_transfer"
            )))
        );
    }

    #[ink::test]
    fn after_token_transfer_should_fail_mint() {
        // Constructor works.
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Can mint token to Alice
        assert!(
            PSP34Mintable::mint(&mut nft, accounts.alice, Id::U8(1u8)).is_ok()
        );
        assert_eq!(PSP34::balance_of(&mut nft, accounts.alice), 1);
        // Turn on error on _after_token_transfer
        nft.change_state_err_on_after();
        // Alice gets an error on _after_token_transfer
        assert_eq!(
            PSP34Mintable::mint(&mut nft, accounts.alice, Id::U8(4u8)),
            Err(PSP34Error::Custom(String::from(
                "Error on _after_token_transfer"
            )))
        );
    }
}
