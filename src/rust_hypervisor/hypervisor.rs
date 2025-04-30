/// hypervisor.rs
///
/// author: Nick Cochran
/// email: nickcochran02@gmail.com
///
/// This file contains the high level structs that hold all components of the hypervisor.


use crate::rust_hypervisor;
use rust_hypervisor::paging;
use paging::*;
use arch_paging::*;
use phys_book::*;
use virt_book::*;

use super::super::user_space_arch::arch::AcdUserLevel;
use super::super::exec::EcdBase;


/// The overall struct that holds everything in the hypervisor.
pub struct RustHypervisor<ACD = AcdUserLevel, ECD = EcdBase> {
    pub rust_hypervisor_header: RustHypervisorHeader,
    pub rust_hypervisor_paging: RustHypervisorPaging<ACD, ECD>
}

impl<ACD, ECD> RustHypervisor<ACD, ECD> {
    pub fn new() -> Self {
        RustHypervisor::<ACD, ECD> {
            rust_hypervisor_header: RustHypervisorHeader::new(),
            rust_hypervisor_paging: RustHypervisorPaging::new(),
        }
    }
}

/// The struct that holds all the paging related components of the hypervisor.
pub struct RustHypervisorPaging<ACD, ECD> {
    pub phys_book: PhysBook,
    pub arch_paging_struct: ArchPaging<ACD, ECD>,
    pub virt_book: VirtBook
}

impl<ACD, ECD> RustHypervisorPaging<ACD, ECD> {
    pub fn new() -> Self {
        RustHypervisorPaging {
            phys_book: PhysBook::new(),
            arch_paging_struct: ArchPaging::<ACD, ECD>::new(),
            virt_book: VirtBook::new()
        }
    }
}

/// The header struct that holds information about the hypervisor.
pub struct RustHypervisorHeader { // from jailhouse
    _signature: [char; 5],
    _arch: u8,
    _flags: usize,
    _max_cpus: usize
}

impl RustHypervisorHeader {
    pub fn new() -> Self {
        RustHypervisorHeader {
            _signature: ['0', '1', '2', '3', '4'], // random value for now
            _arch: 0,
            _flags: 0,
            _max_cpus: 1,
        }
    }
}