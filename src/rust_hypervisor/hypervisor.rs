use crate::rust_hypervisor;
use rust_hypervisor::paging;
use paging::*;
// use arch_paging::*;
// use declarations::*;
use paging_structure::*;
use phys_book::*;
use virt_book::*;

use super::super::user_space_arch::arch::AcdUserLevel;
use super::super::exec::EcdBase;



pub struct RustHypervisor<'hypervisor, ACD = AcdUserLevel, ECD = EcdBase> {
    pub rust_hypervisor_header: RustHypervisorHeader,
    pub rust_hv_memory: RustHypervisorMemory,
    pub rust_hypervisor_paging: RustHypervisorPaging<'hypervisor, ACD, ECD>
}

impl<ACD, ECD> RustHypervisor<'_, ACD, ECD> {
    pub fn new() -> Self {
        RustHypervisor::<ACD, ECD> {
            rust_hypervisor_header: RustHypervisorHeader::new(),
            rust_hv_memory: RustHypervisorMemory::new(),
            rust_hypervisor_paging: RustHypervisorPaging::new(),
        }
    }
}


pub struct RustHypervisorPaging<'mem, ACD, ECD> { // TODO can probably remove 'virt_mem, but figure that out
    pub phys_book: PhysBook,
    pub paging_structure: PagingStructure<ACD, ECD>,
    pub virt_book: VirtBook<'mem>
}

impl<ACD, ECD> RustHypervisorPaging<'_, ACD, ECD> {
    pub fn new() -> Self {
        RustHypervisorPaging {
            phys_book: PhysBook::new(),
            paging_structure: PagingStructure::<ACD, ECD>::new(),
            virt_book: VirtBook::new()
        }
    }
}


pub struct RustHypervisorHeader { // from jailhouse
    signature: [char; 5],
    arch: u8,
    flags: usize,
    max_cpus: usize
}

impl RustHypervisorHeader {
    pub fn new() -> Self {
        RustHypervisorHeader { // FIXME
            signature: ['0', '1', '2', '3', '4'],
            arch: 0,
            flags: 0,
            max_cpus: 1,
        }
    }
}


pub struct RustHypervisorMemory { // from jailhouse
    phys_start: usize,
    virt_start: usize,
    size: usize,
    flags: usize
}

impl RustHypervisorMemory {
    pub fn new() -> Self {
        RustHypervisorMemory { // FIXME
            phys_start: 0,
            virt_start: 0,
            size: 0,
            flags: 0,
        }
    }
}