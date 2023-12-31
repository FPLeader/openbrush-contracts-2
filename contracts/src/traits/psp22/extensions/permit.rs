// Copyright (c) 2012-2023 727-ventures
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

/// Extension of [`PSP22`] that allows create `amount` tokens
/// and assigns them to `account`, increasing the total supply
pub use crate::traits::errors::PSP22Error;
use openbrush::traits::{
    AccountId,
    Balance,
};
pub use openbrush::utils::crypto::Signature;

#[openbrush::wrapper]
pub type PSP22PermitRef = dyn PSP22Permit;

#[openbrush::trait_definition]
pub trait PSP22Permit {
    /// Permit allows `spender` to spend `value` tokens on behalf of `owner` with a signature
    ///
    /// See [`PSP22::_approve`].
    #[ink(message)]
    fn permit(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        value: Balance,
        deadline: u64,
        signature: Signature,
    ) -> Result<(), PSP22Error>;

    #[ink(message)]
    fn domain_separator(&mut self) -> [u8; 32];
}
