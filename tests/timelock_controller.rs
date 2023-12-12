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

#[cfg(feature = "timelock_controller")]
#[pendzl::implementation(TimelockController, AccessControl, AccessControlEnumerable)]
#[ink::contract]
mod timelock_controller {
    use ::ink::env::DefaultEnvironment;
    use ink::{
        codegen::{
            EmitEvent,
            Env,
        },
        env::test::DefaultAccounts,
    };
    use pendzl::{
        test_utils::{
            accounts,
            change_caller,
        },
        traits::Storage,
    };
    /// Emitted when a call is scheduled as part of operation `id`.
    #[ink(event)]
    pub struct CallScheduled {
        #[ink(topic)]
        pub id: OperationId,
        #[ink(topic)]
        pub index: u8,
        pub transaction: Transaction,
        pub predecessor: Option<OperationId>,
        pub delay: Timestamp,
    }

    /// Emitted when a call is performed as part of operation `id`.
    #[ink(event)]
    pub struct CallExecuted {
        #[ink(topic)]
        pub id: OperationId,
        #[ink(topic)]
        pub index: u8,
        pub transaction: Transaction,
    }

    /// Emitted when operation `id` is cancelled.
    #[ink(event)]
    pub struct Cancelled {
        #[ink(topic)]
        pub id: OperationId,
    }

    /// Emitted when the minimum delay for future operations is modified.
    #[ink(event)]
    pub struct MinDelayChange {
        pub old_delay: Timestamp,
        pub new_delay: Timestamp,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct TimelockControllerStruct {
        #[storage_field]
        access_control: access_control::Data,
        #[storage_field]
        enumerable: enumerable::Data,
        #[storage_field]
        timelock: timelock_controller::Data,
    }

    type Event = <TimelockControllerStruct as ::ink::reflect::ContractEventBase>::Type;

    #[overrider(timelock_controller::Internal)]
    fn _emit_min_delay_change_event(&self, old_delay: Timestamp, new_delay: Timestamp) {
        self.env().emit_event(MinDelayChange { old_delay, new_delay })
    }

    #[overrider(timelock_controller::Internal)]
    fn _emit_call_scheduled_event(
        &self,
        id: OperationId,
        index: u8,
        transaction: Transaction,
        predecessor: Option<OperationId>,
        delay: Timestamp,
    ) {
        self.env().emit_event(CallScheduled {
            id,
            index,
            transaction,
            predecessor,
            delay,
        })
    }

    #[overrider(timelock_controller::Internal)]
    fn _emit_cancelled_event(&self, id: OperationId) {
        self.env().emit_event(Cancelled { id })
    }

    #[overrider(timelock_controller::Internal)]
    fn _emit_call_executed_event(&self, id: OperationId, index: u8, transaction: Transaction) {
        self.env().emit_event(CallExecuted { id, index, transaction })
    }

    impl TimelockControllerStruct {
        #[ink(constructor)]
        pub fn new(admin: AccountId, delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {
            let mut instance = Self::default();
            access_control::Internal::_init_with_admin(&mut instance, Some(admin));
            timelock_controller::Internal::_init_with_admin(&mut instance, Some(admin), delay, proposers, executors);
            instance
        }
    }

    fn assert_min_delay_change_event(
        event: &ink::env::test::EmittedEvent,
        expected_old_delay: Timestamp,
        expected_new_delay: Timestamp,
    ) {
        if let Event::MinDelayChange(MinDelayChange { old_delay, new_delay }) =
            <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer")
        {
            assert_eq!(
                old_delay, expected_old_delay,
                "Old delays were not equal: encountered delay {:?}, expected delay {:?}",
                old_delay, expected_old_delay
            );
            assert_eq!(
                new_delay, expected_new_delay,
                "New delays were not equal: encountered delay {:?}, expected delay {:?}",
                new_delay, expected_new_delay
            );
        }
    }

    fn assert_call_scheduled_event(
        event: &ink::env::test::EmittedEvent,
        expected_id: OperationId,
        expected_index: u8,
        expected_transaction: Transaction,
        expected_predecessor: Option<OperationId>,
        expected_delay: Timestamp,
    ) {
        if let Event::CallScheduled(CallScheduled {
            id,
            index,
            transaction,
            predecessor,
            delay,
        }) = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer")
        {
            assert_eq!(
                id, expected_id,
                "Id were not equal: encountered {:?}, expected {:?}",
                id, expected_id
            );
            assert_eq!(
                index, expected_index,
                "Index were not equal: encountered {:?}, expected {:?}",
                index, expected_index
            );
            assert_eq!(
                transaction, expected_transaction,
                "Transaction were not equal: encountered {:?}, expected {:?}",
                transaction, expected_transaction
            );
            assert_eq!(
                predecessor, expected_predecessor,
                "Predecessor were not equal: encountered {:?}, expected {:?}",
                predecessor, expected_predecessor
            );
            assert_eq!(
                delay, expected_delay,
                "Delay were not equal: encountered {:?}, expected {:?}",
                delay, expected_delay
            );
        }
    }

    fn assert_cancelled_event(event: &ink::env::test::EmittedEvent, expected_id: OperationId) {
        if let Event::Cancelled(Cancelled { id }) = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer")
        {
            assert_eq!(
                id, expected_id,
                "Ids were not equal: encountered {:?}, expected {:?}",
                id, expected_id
            );
        }
    }

    fn setup() -> DefaultAccounts<DefaultEnvironment> {
        let accounts = accounts();

        accounts
    }

    #[ink::test]
    fn should_init_with_default_admin() {
        let accounts = setup();
        let timelock = TimelockControllerStruct::new(
            accounts.alice,
            10,
            vec![accounts.bob, accounts.charlie],
            vec![accounts.eve, accounts.charlie],
        );
        assert!(AccessControl::has_role(
            &timelock,
            TIMELOCK_ADMIN_ROLE,
            Some(accounts.alice)
        ));
        assert!(!AccessControl::has_role(&timelock, PROPOSER_ROLE, Some(accounts.alice)));
        assert!(!AccessControl::has_role(&timelock, EXECUTOR_ROLE, Some(accounts.alice)));
        assert_eq!(
            AccessControl::get_role_admin(&timelock, TIMELOCK_ADMIN_ROLE),
            TIMELOCK_ADMIN_ROLE
        );
        assert_eq!(AccessControl::get_role_admin(&timelock, PROPOSER_ROLE), PROPOSER_ROLE);
        assert_eq!(AccessControl::get_role_admin(&timelock, EXECUTOR_ROLE), EXECUTOR_ROLE);
        assert_eq!(TimelockController::get_min_delay(&timelock,), 10);

        assert!(AccessControl::has_role(&timelock, PROPOSER_ROLE, Some(accounts.bob)));
        assert!(AccessControl::has_role(
            &timelock,
            PROPOSER_ROLE,
            Some(accounts.charlie)
        ));
        assert!(!AccessControl::has_role(&timelock, PROPOSER_ROLE, Some(accounts.eve)));
        assert!(AccessControl::has_role(&timelock, EXECUTOR_ROLE, Some(accounts.eve)));
        assert!(AccessControl::has_role(
            &timelock,
            EXECUTOR_ROLE,
            Some(accounts.charlie)
        ));
        assert!(!AccessControl::has_role(&timelock, EXECUTOR_ROLE, Some(accounts.bob)));

        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_min_delay_change_event(&emitted_events[0], 0, 10);
    }

    #[ink::test]
    fn should_schedule() {
        let accounts = setup();
        let min_delay = 10;
        let mut timelock = TimelockControllerStruct::new(accounts.alice, min_delay, vec![accounts.alice], vec![]);

        let id = TimelockController::hash_operation(&mut timelock, Transaction::default(), None, [0; 32]);

        assert!(!TimelockController::is_operation(&mut timelock, id));
        assert!(
            TimelockController::schedule(&mut timelock, Transaction::default(), None, [0; 32], min_delay + 1).is_ok()
        );
        assert!(TimelockController::is_operation(&mut timelock, id));
        assert!(TimelockController::is_operation_pending(&mut timelock, id));
        assert_eq!(TimelockController::get_timestamp(&mut timelock, id), min_delay + 1);

        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_call_scheduled_event(&emitted_events[1], id, 0, Transaction::default(), None, min_delay + 1);
    }

    #[ink::test]
    fn should_schedule_not_proposal() {
        let accounts = setup();
        let min_delay = 10;
        let mut timelock = TimelockControllerStruct::new(accounts.alice, min_delay, vec![], vec![]);

        assert_eq!(
            Err(TimelockControllerError::AccessControlError(
                AccessControlError::MissingRole
            )),
            TimelockController::schedule(&mut timelock, Transaction::default(), None, [0; 32], min_delay + 1)
        );
    }

    #[ink::test]
    fn should_schedule_already_scheduled() {
        let accounts = setup();
        let min_delay = 10;
        let mut timelock = TimelockControllerStruct::new(accounts.alice, min_delay, vec![accounts.alice], vec![]);

        assert!(
            TimelockController::schedule(&mut timelock, Transaction::default(), None, [0; 32], min_delay + 1).is_ok()
        );
        assert_eq!(
            Err(TimelockControllerError::OperationAlreadyScheduled),
            TimelockController::schedule(&mut timelock, Transaction::default(), None, [0; 32], min_delay + 1)
        );
    }

    #[ink::test]
    fn should_schedule_low_delay() {
        let accounts = setup();
        let min_delay = 10;
        let mut timelock = TimelockControllerStruct::new(accounts.alice, min_delay, vec![accounts.alice], vec![]);

        assert_eq!(
            Err(TimelockControllerError::InsufficientDelay),
            TimelockController::schedule(&mut timelock, Transaction::default(), None, [0; 32], min_delay - 1)
        );
    }

    #[ink::test]
    fn should_schedule_batch() {
        let accounts = setup();
        let min_delay = 10;
        let mut timelock = TimelockControllerStruct::new(accounts.alice, min_delay, vec![accounts.alice], vec![]);
        let transactions = vec![Transaction::default(), Transaction::default()];

        let id = TimelockController::hash_operation_batch(&mut timelock, transactions.clone(), None, [0; 32]);

        assert!(!TimelockController::is_operation(&mut timelock, id));
        assert!(
            TimelockController::schedule_batch(&mut timelock, transactions.clone(), None, [0; 32], min_delay + 1)
                .is_ok()
        );
        assert!(TimelockController::is_operation(&mut timelock, id));
        assert!(TimelockController::is_operation_pending(&mut timelock, id));
        assert_eq!(TimelockController::get_timestamp(&mut timelock, id), min_delay + 1);

        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();

        assert_eq!(emitted_events.len(), 3);
        for (i, transaction) in transactions.into_iter().enumerate() {
            assert_call_scheduled_event(&emitted_events[i + 1], id, i as u8, transaction, None, min_delay + 1);
        }
    }

    #[ink::test]
    fn should_schedule_batch_not_proposer() {
        let accounts = setup();
        let min_delay = 10;
        let mut timelock = TimelockControllerStruct::new(accounts.alice, min_delay, vec![], vec![]);
        let transactions = vec![Transaction::default(), Transaction::default()];

        assert_eq!(
            Err(TimelockControllerError::AccessControlError(
                AccessControlError::MissingRole
            )),
            TimelockController::schedule_batch(&mut timelock, transactions.clone(), None, [0; 32], min_delay + 1)
        );
    }

    #[ink::test]
    fn should_cancel() {
        let accounts = setup();
        let min_delay = 10;
        let mut timelock = TimelockControllerStruct::new(accounts.alice, min_delay, vec![accounts.alice], vec![]);

        let id = TimelockController::hash_operation(&mut timelock, Transaction::default(), None, [0; 32]);
        assert!(
            TimelockController::schedule(&mut timelock, Transaction::default(), None, [0; 32], min_delay + 1).is_ok()
        );
        assert!(TimelockController::cancel(&mut timelock, id).is_ok());

        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_call_scheduled_event(&emitted_events[1], id, 0, Transaction::default(), None, min_delay + 1);
        assert_cancelled_event(&emitted_events[2], id);
    }

    #[ink::test]
    fn should_cancel_not_proposer() {
        let accounts = setup();
        let min_delay = 10;
        let mut timelock = TimelockControllerStruct::new(accounts.alice, min_delay, vec![accounts.alice], vec![]);

        let id = TimelockController::hash_operation(&mut timelock, Transaction::default(), None, [0; 32]);
        assert!(
            TimelockController::schedule(&mut timelock, Transaction::default(), None, [0; 32], min_delay + 1).is_ok()
        );

        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_call_scheduled_event(&emitted_events[1], id, 0, Transaction::default(), None, min_delay + 1);

        assert!(AccessControl::revoke_role(&mut timelock, PROPOSER_ROLE, Some(accounts.alice)).is_ok());
        assert_eq!(
            Err(TimelockControllerError::AccessControlError(
                AccessControlError::MissingRole
            )),
            TimelockController::cancel(&mut timelock, id)
        );
    }

    #[ink::test]
    fn should_cancel_not_pending_operation() {
        let accounts = setup();
        let min_delay = 10;
        let mut timelock = TimelockControllerStruct::new(accounts.alice, min_delay, vec![accounts.alice], vec![]);

        let id = TimelockController::hash_operation(&mut timelock, Transaction::default(), None, [0; 32]);
        assert_eq!(
            Err(TimelockControllerError::OperationCannonBeCanceled),
            TimelockController::cancel(&mut timelock, id)
        );
    }

    #[ink::test]
    fn should_update_delay() {
        let accounts = setup();
        let min_delay = 10;
        let mut timelock = TimelockControllerStruct::new(accounts.alice, min_delay, vec![accounts.alice], vec![]);

        // Caller of the method is contract itself
        change_caller(timelock.env().account_id());
        assert!(TimelockController::update_delay(&mut timelock, min_delay + 2).is_ok());
    }

    #[ink::test]
    fn should_update_delay_not_timelock_role() {
        let accounts = setup();
        let min_delay = 10;
        let mut timelock = TimelockControllerStruct::new(accounts.alice, min_delay, vec![accounts.alice], vec![]);
        change_caller([13; 32].into());

        assert_eq!(
            Err(TimelockControllerError::CallerMustBeTimeLock),
            TimelockController::update_delay(&mut timelock, min_delay + 2)
        );
    }
}
