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

#[cfg(feature = "psp22")]
#[pendzl::implementation(PSP22, PSP22Capped, PSP22Mintable)]
#[ink::contract]
mod psp22_capped {
    use pendzl::{
        test_utils::accounts,
        traits::{
            Storage,
            String,
        },
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PSP22Struct {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        cap: capped::Data,
    }

    #[overrider(psp22::Internal)]
    fn _before_token_transfer(
        &mut self,
        from: Option<&AccountId>,
        _to: Option<&AccountId>,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        // `is_none` means that it is minting
        if from.is_none() && capped::Internal::_is_cap_exceeded(self, amount) {
            return Err(PSP22Error::Custom(String::from("Cap exceeded")))
        }
        Ok(())
    }

    impl PSP22Struct {
        /// Constructor which mints `initial_supply` of the token to sender
        /// Will set the token's cap to `cap`
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            assert!(capped::Internal::_init_cap(&mut instance, CAP).is_ok());
            assert!(PSP22Mintable::mint(&mut instance, Self::env().caller(), 1).is_ok());

            instance
        }
    }

    const CAP: u128 = 1000;

    #[ink::test]
    fn initializing_works() {
        let token = PSP22Struct::new();
        assert_eq!(PSP22Capped::cap(&token), CAP);
    }

    #[ink::test]
    fn mint_works() {
        let mut token = PSP22Struct::new();

        let accounts = accounts();
        let alice_balance = PSP22::balance_of(&mut token, accounts.alice);
        assert!(PSP22Mintable::mint(&mut token, accounts.alice, 1).is_ok());
        assert_eq!(PSP22::balance_of(&mut token, accounts.alice), alice_balance + 1);
    }

    #[ink::test]
    fn mint_fails() {
        let mut token = PSP22Struct::new();

        let accounts = accounts();
        let alice_balance = PSP22::balance_of(&mut token, accounts.alice);
        assert_eq!(
            PSP22Mintable::mint(&mut token, accounts.alice, CAP),
            Err(PSP22Error::Custom(String::from("Cap exceeded")))
        );
        assert_eq!(PSP22::balance_of(&mut token, accounts.alice), alice_balance);
    }
}
