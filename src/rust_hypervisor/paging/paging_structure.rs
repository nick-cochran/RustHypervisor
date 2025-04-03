use std::marker::PhantomData;
use crate::exec::EcdBase;
use crate::rust_hypervisor::paging::declarations::PageTable;
use crate::user_space_arch::arch::AcdUserLevel;


pub struct PagingStructure<ACD = AcdUserLevel, ECD = EcdBase> {
    pub page_size: usize,
    pub hv_paging: bool, // TODO taken from Jailhouse and could be useful, but hasn't
    pub root_table: PageTable,
    _ACD_marker: PhantomData<ACD>,
    _ECD_marker: PhantomData<ECD>
}


impl<ACD, ECD> PagingStructure<ACD, ECD> { // TODO check this
    pub fn new() -> PagingStructure<ACD, ECD> {
        PagingStructure {
            page_size: 0,
            hv_paging: true,
            root_table: (0, 0),
            _ACD_marker: PhantomData,
            _ECD_marker: PhantomData,
        }
    }
}

// impl PagingStructure<AcdUserLevel, EcdBase> { // TODO check this as well
//     pub fn new() -> PagingStructure<AcdUserLevel, EcdBase> {
//         PagingStructure {
//             page_size: 0,
//             hv_paging: true,
//             root_table: (0, 0),
//             _ACD_marker: PhantomData,
//             _ECD_marker: PhantomData,
//         }
//     }
// }