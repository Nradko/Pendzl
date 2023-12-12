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
#[pendzl::implementation(PSP34, PSP34Mintable, PSP34Burnable, PSP34Enumerable)]
#[ink::contract]
mod psp34_enumerable {
    use pendzl::{
        contracts::psp34::Id,
        test_utils::{
            accounts,
            change_caller,
        },
        traits::Storage,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct PSP34Struct {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        enumerable: enumerable::Data,
    }

    impl PSP34Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[ink::test]
    fn enumerable_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let nft = PSP34Struct::new();
        // check that alice does not have token by index
        assert_eq!(
            PSP34Enumerable::owners_token_by_index(&nft, accounts.alice, 0u128),
            Err(PSP34Error::TokenNotExists)
        );
        // token by index 1 does not exists
        assert_eq!(
            PSP34Enumerable::token_by_index(&nft, 0u128),
            Err(PSP34Error::TokenNotExists)
        );
    }

    #[ink::test]
    fn enumerable_mint_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Create token Id 1 for Alice
        assert!(psp34::Internal::_mint_to(&mut nft, accounts.alice, Id::U8(1u8)).is_ok());
        // check Alice token by index
        assert_eq!(
            PSP34Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Ok(Id::U8(1u8))
        );
        // check token by index
        assert_eq!(PSP34Enumerable::token_by_index(&mut nft, 0u128), Ok(Id::U8(1u8)));
    }

    #[ink::test]
    fn enumerable_transfer_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Create token Id 1 and Id 2 for Alice
        assert!(psp34::Internal::_mint_to(&mut nft, accounts.alice, Id::U8(1u8)).is_ok());
        assert!(psp34::Internal::_mint_to(&mut nft, accounts.alice, Id::U8(2u8)).is_ok());
        // check Alice token by index
        assert_eq!(
            PSP34Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Ok(Id::U8(1u8))
        );
        assert_eq!(
            PSP34Enumerable::owners_token_by_index(&mut nft, accounts.alice, 1u128),
            Ok(Id::U8(2u8))
        );
        // act. transfer token from alice to bob
        assert!(PSP34::transfer(&mut nft, accounts.bob, Id::U8(1u8), vec![]).is_ok());
        // bob owns token
        assert_eq!(
            PSP34Enumerable::owners_token_by_index(&mut nft, accounts.bob, 0u128),
            Ok(Id::U8(1u8))
        );
        // alice does not own token Id 1
        assert_eq!(
            PSP34Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Ok(Id::U8(2u8))
        );
        assert_eq!(
            PSP34Enumerable::owners_token_by_index(&mut nft, accounts.alice, 1u128),
            Err(PSP34Error::TokenNotExists)
        );
        // act. transfer token from alice to alice
        assert!(PSP34::transfer(&mut nft, accounts.alice, Id::U8(2u8), vec![]).is_ok());
        // check Alice token by index
        assert_eq!(
            PSP34Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Ok(Id::U8(2u8))
        );
    }

    #[ink::test]
    fn token_by_index_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Create token Id 1 for Alice
        assert!(psp34::Internal::_mint_to(&mut nft, accounts.alice, Id::U8(1u8)).is_ok());
        assert!(psp34::Internal::_mint_to(&mut nft, accounts.alice, Id::U8(2u8)).is_ok());
        assert!(psp34::Internal::_mint_to(&mut nft, accounts.alice, Id::U8(3u8)).is_ok());
        // act. transfer token from alice to bob
        assert!(PSP34::transfer(&mut nft, accounts.bob, Id::U8(1u8), vec![]).is_ok());
        assert!(PSP34::transfer(&mut nft, accounts.bob, Id::U8(3u8), vec![]).is_ok());
        change_caller(accounts.bob);
        assert!(PSP34::transfer(&mut nft, accounts.alice, Id::U8(1u8), vec![]).is_ok());
        assert!(PSP34Burnable::burn(&mut nft, accounts.alice, Id::U8(2u8)).is_ok());
        assert!(PSP34::transfer(&mut nft, accounts.alice, Id::U8(3u8), vec![]).is_ok());
        change_caller(accounts.alice);
        assert!(PSP34::transfer(&mut nft, accounts.bob, Id::U8(3u8), vec![]).is_ok());
        // alice does not own token
        assert_eq!(PSP34Enumerable::token_by_index(&mut nft, 0u128), Ok(Id::U8(1u8)));
        assert_eq!(PSP34Enumerable::token_by_index(&mut nft, 1u128), Ok(Id::U8(3u8)));
        assert_eq!(
            PSP34Enumerable::token_by_index(&mut nft, 2u128),
            Err(PSP34Error::TokenNotExists)
        );
    }

    #[ink::test]
    fn enumerable_burn_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(psp34::Internal::_mint_to(&mut nft, accounts.alice, Id::U8(1u8)).is_ok());
        // alice still owns token id 1
        assert_eq!(
            PSP34Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Ok(Id::U8(1u8))
        );
        // index 0 points to token with id 1
        assert_eq!(PSP34Enumerable::token_by_index(&mut nft, 0u128), Ok(Id::U8(1u8)));
        // Destroy token Id 1.
        assert!(PSP34Burnable::burn(&mut nft, accounts.alice, Id::U8(1u8)).is_ok());
        // alice does not owns any tokens
        assert_eq!(
            PSP34Enumerable::owners_token_by_index(&mut nft, accounts.alice, 0u128),
            Err(PSP34Error::TokenNotExists)
        );
        // token by index 1 does not exists
        assert_eq!(
            PSP34Enumerable::token_by_index(&mut nft, 0u128),
            Err(PSP34Error::TokenNotExists)
        );
    }
}
