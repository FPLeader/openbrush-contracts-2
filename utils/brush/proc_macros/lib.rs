#![feature(drain_filter)]
extern crate proc_macro;
mod internal;
mod contract;
mod trait_definition;
mod metadata;
mod modifier_definition;
mod modifiers;

use proc_macro::TokenStream;

/// Entry point for use brush's macros in ink! smart contracts.
///
/// # Description
///
/// The macro consumes brush's macros to simplify the usage of the library.
/// After consumption, it pastes ink! code and then ink!'s macros will be processed.
///
/// This macro consumes impl section for traits defined with [`#[brush::trait_definition]`](`macro@crate::trait_definition`).
///
/// Also, this macro marks each non-ink! implementation section with `#[cfg(not(feature = "ink-as-dependency"))]`.
#[proc_macro_attribute]
pub fn contract(_attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    contract::generate(_attrs, ink_module)
}

/// Defines extensible trait in the scope of brush::contract.
/// It is a common rust trait, so you can use any features of rust inside of this trait.
/// If this trait contains some methods marked with `#[ink(message)]` or `#[ink(constructor)]` attributes,
/// this macro will extract these attributes and will put them into a separate trait
/// (the separate trait only is used to call methods from the original trait), but the macro will not touch methods.
///
/// This macro stores definition of the trait in a temporary file during build process.
/// Based on this definition [`#[brush::contract]`](`macro@crate::contract`)
/// will generate implementation of additional traits.
///
///  ** Note ** The name of the trait defined via this macro must be unique for the whole project.
///
/// # Example: Definition
///
/// ```
/// pub use ink_storage::{
///     collections::{
///         HashMap as StorageHashMap,
///     },
/// };
/// use brush::traits::{AccountId, Balance, InkStorage};
///
/// #[derive(Default, Debug)]
/// pub struct Data {
///     pub balances: StorageHashMap<AccountId, Balance>,
/// }
///
/// pub trait PSP22Storage: InkStorage {
///     fn get(&self) -> &Data;
///     fn get_mut(&mut self) -> &mut Data;
/// }
///
/// #[brush::trait_definition]
/// pub trait PSP22: PSP22Storage {
///     /// Returns the account Balance for the specified `owner`.
///     #[ink(message)]
///     fn balance_of(&self, owner: AccountId) -> Balance {
///         self.get().balances.get(&owner).copied().unwrap_or(0)
///     }
///
///     /// Transfers `value` amount of tokens from the caller's account to account `to`.
///     #[ink(message)]
///     fn transfer(&mut self, to: AccountId, value: Balance) {
///         self._transfer_from_to(to, to, value)
///     }
///
///     fn _transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Balance) {
///         let from_balance = self.balance_of(from);
///         assert!(from_balance >= amount, "InsufficientBalance");
///         self.get_mut().balances.insert(from, from_balance - amount);
///         let to_balance = self.balance_of(to);
///         self.get_mut().balances.insert(to, to_balance + amount);
///     }
/// }
/// ```
///
/// # Example: Implementation
///
/// It uses storage trait from above.
///
/// ```
/// #[brush::contract]
/// mod base_psp20 {
///     pub use ink_storage::collections::{HashMap as StorageHashMap};
///
///     #[brush::storage_trait]
///     pub trait PSP22ExampleStorage {
///         fn _supply(&self) -> & Balance;
///         fn _supply_mut(&mut self) -> &mut Balance;
///
///         fn _balances(&self) -> & StorageHashMap<AccountId, Balance>;
///         fn _balances_mut(&mut self) -> &mut StorageHashMap<AccountId, Balance>;
///
///         fn _allowances(&self) -> & StorageHashMap<(AccountId, AccountId), Balance>;
///         fn _allowances_mut(&mut self) -> &mut StorageHashMap<(AccountId, AccountId), Balance>;
///     }
///
///     #[brush::trait_definition]
///     pub trait PSP22Example: PSP22ExampleStorage {
///         /// Returns the account Balance for the specified `owner`.
///         #[ink(message)]
///         fn balance_of(&self, owner: AccountId) -> Balance {
///             self._balances().get(&owner).copied().unwrap_or(0)
///         }
///
///         /// Transfers `value` amount of tokens from the caller's account to account `to`.
///         #[ink(message)]
///         fn transfer(&mut self, to: AccountId, value: Balance) {
///             let from = Self::env().caller();
///             self._transfer_from_to(from, to, value)
///         }
///
///         fn _transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Balance) {
///             let from_balance = self.balance_of(from);
///             assert!(from_balance >= amount, "InsufficientBalance");
///             self._balances_mut().insert(from, from_balance - amount);
///             let to_balance = self.balance_of(to);
///             self._balances_mut().insert(to, to_balance + amount);
///         }
///     }
///
///     #[ink(storage)]
///     #[derive(Default, PSP22ExampleStorage)]
///     pub struct PSP22Struct {
///         hated_account: AccountId,
///     }
///
///     impl PSP22Example for PSP22Struct {
///         // Let's override method to reject transactions to bad account
///         fn _transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Balance) {
///             assert!(to != self.hated_account, "I hate this account!");
///             #[super]self._transfer_from_to(from, to, amount);
///         }
///     }
///
///     impl PSP22Struct {
///         #[ink(constructor)]
///         pub fn new(hated_account: AccountId) -> Self {
///             let mut instance = Self::default();
///             instance.hated_account = hated_account;
///             instance
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn trait_definition(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    trait_definition::generate(_attrs, _input)
}

/// This macro only checks that some free-standing function satisfies a set of rules.
///
/// Rules:
/// - First argument should not be `self`.
/// - First argument must be a reference to a type `instance: &T`. In most cases it's the instance of contract.
/// - Second argument is function's body(this function contains the main code of method attached to the modifier).
/// The type must be `Fn(&T)` or `FnMut(&T)`.
/// - Every next argument should not be references to object.
/// Because modifier allows only to pass arguments by value(Modifier will pass the clone of argument).
/// - The return type of body function(second argument) must be the same as the return type of modifier.
///
/// # Example: Definition
///
/// ```
/// #[derive(Default)]
/// struct Contract {
///     initialized: bool,
/// }
///
/// #[brush::modifier_definition]
/// fn once<BodyFn: Fn(&mut Contract)>(instance: &mut Contract, body: BodyFn, _example_data: u8) {
///     assert!(!instance.initialized, "Contract is already initialized");
///     body(instance);
///     instance.initialized = true;
/// }
/// ```
#[proc_macro_attribute]
pub fn modifier_definition(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    modifier_definition::generate(_attrs, _input)
}

/// Macro calls every modifier function by passing self and the code of function's body.
/// It means that modifiers must be available in the scope of the marked method.
///
/// Modifiers are designed to be used for methods in impl sections.
/// The method can have several modifiers. They will be expanded from left to right.
/// The modifier can accept arguments from the scope of the method definition
/// (you can pass an argument from the signature of marked method or from the outside scope of function).
/// The modifier accepts arguments only by value and the type of argument must support `Clone` trait,
/// because macro will clone the argument and will pass it to the modifier.
///
/// # Explanation:
///
/// Let's define next modifiers.
/// ```
/// #[brush::modifier_definition]
/// fn A<T>(instance: &T, body: impl Fn(&T) -> &'static str) -> &'static str {
///     println!("A before");
///     let result = body(instance);
///     println!("A after");
///     result
/// }
///
/// #[brush::modifier_definition]
/// fn B<T, F: Fn(&T) -> &'static str>(instance: &T, body: F, data: u8) -> &'static str {
///     println!("B before {}", data);
///     let result = body(instance);
///     println!("B after {}", data);
///     result
/// }
///
/// #[brush::modifier_definition]
/// fn C<T, F>(instance: &T, body: F) -> &'static str
///     where F: Fn(&T) -> &'static str
/// {
///     println!("C before");
///     let result = body(instance);
///     println!("C after");
///     result
/// }
///
/// struct Contract {}
///
/// impl Contract {
///     #[brush::modifiers(A, B(_data), C)]
///     fn main_logic(&self, _data: u8) -> &'static str {
///         return "Return value"
///     }
/// }
/// ```
/// The code above will be expanded into:
/// ```
/// #[brush::modifier_definition]
/// fn A<T>(instance: &T, body: impl Fn(&T) -> &'static str) -> &'static str {
///     println!("A before");
///     let result = body(instance);
///     println!("A after");
///     result
/// }
///
/// #[brush::modifier_definition]
/// fn B<T, F: Fn(&T) -> &'static str>(instance: &T, body: F, data: u8) -> &'static str {
///     println!("B before {}", data);
///     let result = body(instance);
///     println!("B after {}", data);
///     result
/// }
///
/// #[brush::modifier_definition]
/// fn C<T, F>(instance: &T, body: F) -> &'static str
///     where F: Fn(&T) -> &'static str
/// {
///     println!("C before");
///     let result = body(instance);
///     println!("C after");
///     result
/// }
///
/// struct Contract {}
///
/// impl Contract {
///     fn main_logic(&self, _data: u8) -> &'static str {
///         let mut __brush_body_2 = |__brush_instance_modifier: &Self| {
///             let __brush_cloned_0 = _data.clone();
///             let mut __brush_body_1 = |__brush_instance_modifier: &Self| {
///                 let mut __brush_body_0 = |__brush_instance_modifier: &Self| return "Return value";
///                 C(__brush_instance_modifier, __brush_body_0)
///             };
///             B(__brush_instance_modifier, __brush_body_1, __brush_cloned_0)
///         };
///         A(self, __brush_body_2)
///     }
/// }
///
/// ```
///
/// # Example: Usage
///
/// ```
/// #[brush::contract]
/// mod example {
///     #[ink(storage)]
///     #[derive(Default)]
///     pub struct Contract {
///         initialized: bool,
///         owner: AccountId,
///     }
///
///     #[brush::modifier_definition]
///     fn once(instance: &mut Contract, body: impl Fn(&mut Contract)) {
///         assert!(!instance.initialized, "Contract is already initialized");
///         body(instance);
///         instance.initialized = true;
///     }
///
///     impl Contract {
///         #[ink(constructor)]
///         pub fn new() -> Self {
///             Self::default()
///         }
///
///         #[ink(message)]
///         #[brush::modifiers(once)]
///         pub fn init(&mut self, owner: AccountId) {
///             self.owner = owner;
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn modifiers(_attrs: TokenStream, method: TokenStream) -> TokenStream {
    modifiers::generate(_attrs, method)
}
