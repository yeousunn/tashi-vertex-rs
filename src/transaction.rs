use std::ffi::c_void;
use std::mem::{self, MaybeUninit};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::slice::{from_raw_parts, from_raw_parts_mut};

use crate::Engine;
use crate::error::TVResult;

// FIXME: add Drop impl to free transaction memory if not sent
#[must_use]
pub struct Transaction {
    data: NonNull<u8>,
    size: usize,
}

impl Transaction {
    /// Allocates a buffer for a transaction of the specified size.
    pub fn allocate(size: usize) -> Self {
        let mut data = MaybeUninit::<NonNull<u8>>::uninit();

        unsafe { tv_transaction_allocate(size, data.as_mut_ptr().cast()) }.assert_ok();

        let data = unsafe { data.assume_init() };

        Self { data, size }
    }

    /// Sends the transaction to the network via the specified engine.
    pub(crate) fn send(self, engine: &Engine) -> crate::Result<()> {
        let data = self.data.as_ptr();
        let size = self.size;

        #[allow(clippy::forget_non_drop)]
        mem::forget(self);

        unsafe { tv_transaction_send(engine.handle.as_ptr(), data, size) }.ok()
    }
}

impl Deref for Transaction {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { from_raw_parts(self.data.as_ptr(), self.size) }
    }
}

impl DerefMut for Transaction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { from_raw_parts_mut(self.data.as_ptr(), self.size) }
    }
}

unsafe extern "C" {
    fn tv_transaction_allocate(size: usize, data: *mut *mut c_void) -> TVResult;

    fn tv_transaction_send(engine: *mut c_void, data: *const u8, size: usize) -> TVResult;
}
