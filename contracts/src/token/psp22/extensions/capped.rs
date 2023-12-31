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

pub use crate::{
    psp22,
    psp22::extensions::capped,
    traits::psp22::{
        extensions::capped::*,
        *,
    },
};
pub use capped::Internal as _;
use openbrush::traits::{
    Balance,
    Storage,
    String,
};
pub use psp22::{
    Internal as _,
    InternalImpl as _,
    PSP22Impl,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub cap: Balance,
}

pub trait PSP22CappedImpl: Internal {
    fn cap(&self) -> Balance {
        self._cap()
    }
}

pub trait Internal {
    /// Initializes the token's cap
    fn _init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error>;

    fn _is_cap_exceeded(&self, amount: &Balance) -> bool;

    fn _cap(&self) -> Balance;
}

pub trait InternalImpl: Storage<Data> + Internal + PSP22 {
    fn _init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error> {
        if cap == 0 {
            return Err(PSP22Error::Custom(String::from("Cap must be above 0")))
        }
        self.data().cap.set(&cap);
        Ok(())
    }

    fn _is_cap_exceeded(&self, amount: &Balance) -> bool {
        if self.total_supply() + amount > Internal::_cap(self) {
            return true
        }
        false
    }

    fn _cap(&self) -> Balance {
        self.data().cap.get_or_default()
    }
}
