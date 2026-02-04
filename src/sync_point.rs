use std::ffi::c_void;

use crate::ptr::Pointer;

/// A sync point describes a decision or action related to the management of
/// the consensus engine which a super-majority of peers agreed upon.
pub struct SyncPoint {
    #[allow(unused)]
    pub(crate) handle: Pointer<TVSyncPoint>,
}

pub(crate) type TVSyncPoint = c_void;
