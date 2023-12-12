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

#[cfg(feature = "psp37")]
#[pendzl::implementation(PSP37, PSP37Burnable)]
#[ink::contract]
mod psp37_burnable {
    use pendzl::{
        test_utils::accounts,
        traits::{
            Storage,
            String,
        },
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct PSP37Struct {
        #[storage_field]
        psp37: psp37::Data,
        // field for testing _before_token_transfer
        return_err_on_before: bool,
        // field for testing _after_token_transfer
        return_err_on_after: bool,
    }

    #[overrider(psp37::Internal)]
    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _ids: &[(Id, Balance)],
    ) -> Result<(), PSP37Error> {
        if self.return_err_on_before {
            return Err(PSP37Error::Custom(String::from("Error on _before_token_transfer")))
        }
        Ok(())
    }

    #[overrider(psp37::Internal)]
    fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _ids: &[(Id, Balance)],
    ) -> Result<(), PSP37Error> {
        if self.return_err_on_after {
            return Err(PSP37Error::Custom(String::from("Error on _after_token_transfer")))
        }
        Ok(())
    }

    impl PSP37Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint(&mut self, acc: AccountId, id: Id, amount: Balance) -> Result<(), PSP37Error> {
            psp37::Internal::_mint_to(self, acc, vec![(id, amount)])
        }

        pub fn change_state_err_on_before(&mut self) {
            self.return_err_on_before = !self.return_err_on_before;
        }

        pub fn change_state_err_on_after(&mut self) {
            self.return_err_on_after = !self.return_err_on_after;
        }
    }

    #[ink::test]
    fn burn_works() {
        let token_id1 = Id::U128(1);
        let token_id2 = Id::U128(2);
        let token_amount1 = 1;
        let token_amount2 = 10;
        let accounts = accounts();

        let mut nft = PSP37Struct::new();
        assert!(nft.mint(accounts.alice, token_id1.clone(), token_amount1).is_ok());
        assert!(nft.mint(accounts.alice, token_id2.clone(), token_amount2).is_ok());

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 2);
        assert_eq!(PSP37::total_supply(&mut nft, None), 2);

        assert!(nft.mint(accounts.bob, token_id2.clone(), token_amount2).is_ok());

        assert_eq!(PSP37::total_supply(&mut nft, None), 2);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id1.clone())), 1);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id2.clone())), 20);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 2);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, None), 1);

        assert!(PSP37Burnable::burn(
            &mut nft,
            accounts.bob,
            vec![(token_id2.clone(), token_amount2), (token_id1.clone(), 0)]
        )
        .is_ok());

        assert_eq!(PSP37::total_supply(&mut nft, None), 2);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id2.clone())), 10);

        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 2);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.bob, None), 0);

        assert!(PSP37Burnable::burn(&mut nft, accounts.alice, vec![(token_id2.clone(), token_amount2)]).is_ok());

        assert_eq!(PSP37::total_supply(&mut nft, None), 1);
        assert_eq!(PSP37::total_supply(&mut nft, Some(token_id2.clone())), 0);
        assert_eq!(PSP37::balance_of(&mut nft, accounts.alice, None), 1);
    }

    #[ink::test]
    fn burn_insufficient_balance() {
        let token_id_1 = Id::U128(1);
        let burn_amount = 2;
        let accounts = accounts();

        let mut nft = PSP37Struct::new();

        assert_eq!(
            PSP37Burnable::burn(&mut nft, accounts.alice, vec![(token_id_1, burn_amount)]),
            Err(PSP37Error::InsufficientBalance),
        );
    }

    #[ink::test]
    fn before_token_transfer_should_fail_burn() {
        let accounts = accounts();
        let token_id = Id::U128(123);
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        assert!(nft.mint(accounts.alice, token_id.clone(), 2).is_ok());
        // Alice can burn tokens
        assert!(PSP37Burnable::burn(&mut nft, accounts.alice, vec![(token_id.clone(), 1)]).is_ok());
        // Turn on error on _before_token_transfer
        nft.change_state_err_on_before();
        // Alice gets an error on _before_token_transfer
        assert_eq!(
            PSP37Burnable::burn(&mut nft, accounts.alice, vec![(token_id.clone(), 1)]),
            Err(PSP37Error::Custom(String::from("Error on _before_token_transfer")))
        );
    }

    #[ink::test]
    fn after_token_transfer_should_fail_burn() {
        let accounts = accounts();
        let token_id = Id::U128(123);
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        assert!(nft.mint(accounts.alice, token_id.clone(), 2).is_ok());
        // Alice can burn tokens
        assert!(PSP37Burnable::burn(&mut nft, accounts.alice, vec![(token_id.clone(), 1)]).is_ok());
        // Turn on error on _after_token_transfer
        nft.change_state_err_on_after();
        // Alice gets an error on _after_token_transfer
        assert_eq!(
            PSP37Burnable::burn(&mut nft, accounts.alice, vec![(token_id.clone(), 1)]),
            Err(PSP37Error::Custom(String::from("Error on _after_token_transfer")))
        );
    }
}
