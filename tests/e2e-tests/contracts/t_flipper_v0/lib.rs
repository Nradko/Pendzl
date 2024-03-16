#![cfg_attr(not(feature = "std"), no_std, no_main)]
use ink::prelude::string::{String, ToString};

#[ink::event]
pub struct Flipped {
    #[ink(topic)]
    pub new_value: bool,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FlipperError {
    SomeError(String),
    SomeError2,
    SomeError3,
}

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct SomeStructInner {
    pub x: bool,
    pub y: u128,
}

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct SomeStruct {
    pub a: bool,
    pub inner: SomeStructInner,
}

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct FlipUpgradeableStorageItem {
    pub val_v0: u128,
    #[lazy]
    pub struct_v0: SomeStruct,
}

#[pendzl::implementation(AccessControl)]
#[ink::contract]
mod t_flipper {
    use crate::*;

    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct Flipper {
        value: bool,
        #[storage_field]
        access: AccessControlData,
        #[storage_field]
        upgradeable: FlipUpgradeableStorageItem,
    }

    impl Flipper {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            let mut init_upgradeable = FlipUpgradeableStorageItem {
                val_v0: 1337_u128,
                struct_v0: Default::default(),
            };
            init_upgradeable.struct_v0.set(&SomeStruct {
                a: true,
                inner: SomeStructInner { x: false, y: 42 },
            });
            Self {
                access: Default::default(),
                value: init_value,
                upgradeable: init_upgradeable,
            }
            // Self {
            //     // value: init_value,
            //     access: Default::default(),
            // }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
            self.env().emit_event(Flipped {
                new_value: self.value,
            });
        }

        #[ink(message)]
        pub fn flip_and_return_value(&mut self) -> Result<u128, FlipperError> {
            self.value = !self.value;
            self.env().emit_event(Flipped {
                new_value: self.value,
            });
            Ok(5)
        }

        #[ink(message)]
        pub fn return_error(&mut self) -> Result<u128, FlipperError> {
            Err(FlipperError::SomeError("Some error".to_string()))
        }

        #[ink(message)]
        pub fn do_panic(&mut self) {
            panic!("Some error")
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            // self.value
            true
        }
    }
}
