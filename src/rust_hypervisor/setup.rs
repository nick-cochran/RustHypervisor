use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use lazy_static::lazy_static;
use crate::rust_hypervisor;
use rust_hypervisor::paging::paging_init;
use rust_hypervisor::hypervisor::*;
use crate::exec::EcdBase;
use crate::rust_hypervisor::paging::arch_paging::ArchPaging;
use crate::user_space_arch::arch::AcdUserLevel;


type GlobalUserSpaceHypervisor<'hypervisor> = Arc<Mutex<RustHypervisor<'hypervisor, AcdUserLevel, EcdBase>>>;

// this will be changed later on to call something that uses the correct typing depending on arch
type GlobalHypervisor<'hypervisor> = GlobalUserSpaceHypervisor<'hypervisor>;

static INVALID_CPU_ID: u32 = !0u32;
static NULL_PAGE_SIZE: usize = 0;

static HV_CPU_ID: AtomicU32 = AtomicU32::new(INVALID_CPU_ID);





lazy_static! {
    /// Global Arc Mutex to hold everything I need
    pub static ref HYPERVISOR : GlobalHypervisor<'static> = Arc::new(Mutex::new(RustHypervisor::new()));
}

// FIXME to be set by arch --> in theory this is right
pub static PAGE_SIZE : AtomicUsize = AtomicUsize::new(NULL_PAGE_SIZE);



/// This is where the hypervisor setup starts from the architecture
pub fn hv_init(cpu_id: u32) -> Result<(), u8> {

    PAGE_SIZE.store(
        HYPERVISOR.lock().unwrap().rust_hypervisor_paging.paging_structure.get_arch_page_size(),
        Ordering::SeqCst);

    // initialize the hypervisor on only one CPU
    if HV_CPU_ID.load(Ordering::SeqCst) == INVALID_CPU_ID {
        init_hv_system(cpu_id).expect("temp");
    }



    Ok(())
}





/// initialize the hypervisor system
fn init_hv_system(cpu_id: u32) -> Result<(), u8> {

    HV_CPU_ID.store(cpu_id, Ordering::SeqCst);

    paging_init()

}