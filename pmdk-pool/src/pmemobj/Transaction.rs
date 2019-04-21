// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.

/// Represents a transaction.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Transaction;

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum TxStage {
    None = pobj_tx_stage_TX_STAGE_NONE,
    Work = pobj_tx_stage_TX_STAGE_WORK,
    OnCommit = pobj_tx_stage_TX_STAGE_ONCOMMIT,
    OnAbort = pobj_tx_stage_TX_STAGE_ONABORT,
    Finally = pobj_tx_stage_TX_STAGE_FINALLY,
}

pub fn txstage_from_u32(n: u32) -> Option<TxStage> {
    if n >= pobj_tx_stage_TX_STAGE_NONE && n < pobj_tx_stage_MAX_TX_STAGE {
        Some(unsafe { ::std::mem::transmute(n) })
    } else {
        None
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        unsafe {
            if pmemobj_tx_stage() == pobj_tx_stage_TX_STAGE_WORK {
                pmemobj_tx_abort(ECANCELED)
            }
            pmemobj_tx_end();
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum TxError {
        FailToStart {
            description("Fail to start transaction")
        }
        WrongStage {
            description("Wrong stage, should be in work")
        }
        Aborted {
            description("Transaction aborted")
        }
        EndPrematurely {
            description("Transaction ended prematurely")
        }
        Other(desc: String)
    }
}

impl Transaction {
    pub fn new(pool: &mut ObjectPool) -> Result<(), TxError> {
        unsafe {
            if pmemobj_tx_begin(pool.0, ::std::ptr::null_mut(), pobj_tx_param_TX_PARAM_NONE) != 0 {
                return Err(TxError::FailToStart);
            }
        }
        Ok(())
    }

    pub fn run<F, R>(pool: &mut ObjectPool, f: F) -> Result<R, TxError>
    where
        F: FnOnce() -> R,
    {
        if unsafe { pmemobj_tx_begin(pool.0, ::std::ptr::null_mut(), pobj_tx_param_TX_PARAM_NONE) }
            != 0
        {
            return Err(TxError::FailToStart);
        }

        let res = f();
        unsafe {
            let stage = pmemobj_tx_stage();
            if stage == pobj_tx_stage_TX_STAGE_WORK {
                pmemobj_tx_commit();
            } else if stage == pobj_tx_stage_TX_STAGE_ONABORT {
                pmemobj_tx_end();
                return Err(TxError::Aborted);
            } else if stage == pobj_tx_stage_TX_STAGE_NONE {
                return Err(TxError::EndPrematurely);
            }
            pmemobj_tx_end();
        }
        return Ok(res);
    }

    pub fn abort(err: i32) -> Result<(), TxError> {
        unsafe {
            if pmemobj_tx_stage() != pobj_tx_stage_TX_STAGE_WORK {
                return Err(TxError::WrongStage);
            }
            pmemobj_tx_abort(err);
        }
        Ok(())
    }

    pub fn commit() -> Result<(), TxError> {
        unsafe {
            if (pmemobj_tx_stage() != pobj_tx_stage_TX_STAGE_WORK) {
                return Err(TxError::WrongStage);
            }
            pmemobj_tx_commit();
        }
        Ok(())
    }
}

#[inline(always)]
fn set_error_number_if_necessary(os_error_number: c_int) {
    if unlikely(os_error_number != 0) {
        set_errno(Errno(os_error_number));
    }
}
