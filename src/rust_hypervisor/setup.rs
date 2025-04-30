/// setup.rs
///
/// author: Nick Cochran
/// email: nickcochran02@gmail.com
///
/// This file contains setup functions that initialize the hypervisor.

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use lazy_static::lazy_static;
use crate::rust_hypervisor;
use rust_hypervisor::paging::paging_init;
use rust_hypervisor::hypervisor::*;
use crate::exec::EcdBase;
use crate::rust_hypervisor::paging::arch_paging_access::ArchPagingAccess;
use crate::user_space_arch::arch::AcdUserLevel;


type GlobalUserSpaceHypervisor = Arc<Mutex<RustHypervisor<AcdUserLevel, EcdBase>>>;

/// This would be changed based on the build target in Cargo
/// to target specific architectures and execution environments
type GlobalHypervisor = GlobalUserSpaceHypervisor;

static INVALID_CPU_ID: usize = !0usize;
static NULL_PAGE_SIZE: usize = 0;

static HV_CPU_ID: AtomicUsize = AtomicUsize::new(INVALID_CPU_ID);

lazy_static! {
    /// Global Arc Mutex to hold everything in the Hypervisor
    pub static ref HYPERVISOR : GlobalHypervisor = Arc::new(Mutex::new(RustHypervisor::new()));
}

// Store page size in an Atomic to hold it globally and be set by architecture
pub static PAGE_SIZE : AtomicUsize = AtomicUsize::new(NULL_PAGE_SIZE);



/// Initialize all parts of the hypervisor by calling other init functions.
pub fn hv_init(cpu_id: usize) -> Result<(), u8> {

    // store the page size in the global atomic
    // from the call to the arch_paging struct's get_arch_page_size
    PAGE_SIZE.store(
        HYPERVISOR.lock().unwrap().rust_hypervisor_paging.arch_paging_struct.get_arch_page_size(),
        Ordering::SeqCst);

    // initialize the hypervisor itself on only one CPU
    if HV_CPU_ID.load(Ordering::SeqCst) == INVALID_CPU_ID {
        init_hv_system(cpu_id).expect("Hypervisor Initialization Error");
    }


    Ok(())
}

/// Initializes the hypervisor system.
fn init_hv_system(cpu_id: usize) -> Result<(), u8> {

    HV_CPU_ID.store(cpu_id, Ordering::SeqCst);

    // initialize the paging part of the hypervisor
    paging_init()

}