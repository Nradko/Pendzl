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

#[cfg(feature = "pausable")]
#[openbrush::implementation(Pausable)]
#[openbrush::contract]
mod pausable {
    use ::ink::env::DefaultEnvironment;
    use ink::{
        codegen::{
            EmitEvent,
            Env,
        },
        env::test::DefaultAccounts,
    };
    use openbrush::{
        test_utils::accounts,
        traits::Storage,
    };

    /// Emitted when the pause is triggered by `account`.
    #[ink(event)]
    pub struct Paused {
        pub account: AccountId,
    }

    /// Emitted when the pause is lifted by `account`.
    #[ink(event)]
    pub struct Unpaused {
        pub account: AccountId,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyFlipper {
        #[storage_field]
        pause: Data,
        flipped: bool,
    }

    impl MyFlipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        #[openbrush::modifiers(when_paused)]
        pub fn flip(&mut self) -> Result<bool, PausableError> {
            let previous = self.flipped;
            self.flipped = !previous;

            Ok(previous)
        }
    }

    #[overrider(pausable::Internal)]
    fn _emit_paused_event(&self, account: AccountId) {
        self.env().emit_event(Paused { account })
    }

    #[overrider(pausable::Internal)]
    fn _emit_unpaused_event(&self, account: AccountId) {
        self.env().emit_event(Unpaused { account })
    }

    type Event = <MyFlipper as ::ink::reflect::ContractEventBase>::Type;

    fn assert_paused_event(event: &ink::env::test::EmittedEvent, expected_account: AccountId) {
        if let Event::Paused(Paused { account }) = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer")
        {
            assert_eq!(
                account, expected_account,
                "Accounts were not equal: encountered {:?}, expected {:?}",
                account, expected_account
            );
        }
    }

    fn assert_unpaused_event(event: &ink::env::test::EmittedEvent, expected_account: AccountId) {
        if let Event::Unpaused(Unpaused { account }) = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer")
        {
            assert_eq!(
                account, expected_account,
                "Accounts were not equal: encountered {:?}, expected {:?}",
                account, expected_account
            );
        }
    }

    fn setup() -> DefaultAccounts<DefaultEnvironment> {
        let accounts = accounts();

        accounts
    }

    #[ink::test]
    fn pause_works() {
        let accounts = setup();
        let mut inst = MyFlipper::new();
        assert!(pausable::Internal::_pause(&mut inst).is_ok());
        assert!(inst.pause.paused.get_or_default());

        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_paused_event(&emitted_events[0], accounts.alice);
    }

    #[ink::test]
    fn double_pause_fails() {
        let mut inst = MyFlipper::new();
        assert!(pausable::Internal::_pause(&mut inst).is_ok());
        assert_eq!(Err(PausableError::Paused), pausable::Internal::_pause(&mut inst));
    }

    #[ink::test]
    fn flip_works() {
        let mut inst = MyFlipper::new();
        assert!(pausable::Internal::_pause(&mut inst).is_ok());

        assert_eq!(Ok(false), inst.flip());
        assert_eq!(Ok(true), inst.flip());
        assert_eq!(Ok(false), inst.flip());
    }

    #[ink::test]
    fn flip_fails() {
        let mut inst = MyFlipper::new();

        assert_eq!(Err(PausableError::NotPaused), inst.flip());
    }

    #[ink::test]
    fn unpause_fails() {
        let mut inst = MyFlipper::new();

        assert_eq!(Err(PausableError::NotPaused), pausable::Internal::_unpause(&mut inst));
    }

    #[ink::test]
    fn unpause_works() {
        let accounts = setup();
        let mut inst = MyFlipper::new();

        assert!(pausable::Internal::_pause(&mut inst).is_ok());
        assert!(pausable::Internal::_unpause(&mut inst).is_ok());
        assert!(!inst.pause.paused.get_or_default());

        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_unpaused_event(&emitted_events[0], accounts.alice);
    }

    #[ink::test]
    fn switch_pause_works() {
        let accounts = setup();
        let mut inst = MyFlipper::new();

        assert!(pausable::Internal::_pause(&mut inst).is_ok());
        assert!(pausable::Internal::_switch_pause(&mut inst).is_ok());
        assert!(!inst.pause.paused.get_or_default());

        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_unpaused_event(&emitted_events[0], accounts.alice);
    }
}
