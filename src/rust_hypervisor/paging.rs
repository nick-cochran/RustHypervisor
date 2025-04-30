/// paging.rs
///
/// author: Nick Cochran
/// email: nickcochran02@gmail.com
///
/// This file contains all the relevant paging functions to allocate and free pages.

pub mod declarations;
pub mod phys_book;
pub mod virt_book;
pub mod arch_paging;
pub mod arch_paging_access;

use std::sync::atomic::Ordering;
use crate::rust_hypervisor;
use arch_paging_access::*;
use declarations::*;
use phys_book::*;
use rust_hypervisor::setup::HYPERVISOR;
use crate::rust_hypervisor::setup::PAGE_SIZE;


/// Initialize the hypervisor paging system and all structs used by it.
///
/// # Returns
///
/// * `Ok(())` - if the initialization was successful
/// * `Err(u8)` - if the initialization failed
pub fn paging_init() -> Result<(), u8> {

    let mut hypervisor_struct = HYPERVISOR.lock().unwrap();

    // get references to the relevant structs
    let paging_structs = &mut hypervisor_struct.rust_hypervisor_paging;
    let phys_book = &mut paging_structs.phys_book;
    let virt_book = &mut paging_structs.virt_book;
    let arch_paging  = &mut paging_structs.arch_paging_struct;


    // set up the variables inside of phys_book and hv_paging

    let page_size : usize = PAGE_SIZE.load(Ordering::SeqCst);
    arch_paging.page_size = page_size;

    let mem_size : usize = arch_paging.get_hv_mem_size();
    let num_total_pages : NumPages = mem_size / page_size;
    phys_book.num_total_pages = num_total_pages;

    // This could be changed later to just be an offset starting at 0
    // where the arch code knows the starting address and applies the offset to that
    let start_addr = arch_paging.get_hv_mem_start();
    phys_book.mem_start = start_addr;
    phys_book.free_pages.insert(start_addr, num_total_pages); // insert the whole thing into the free pages map

    // This is where any relevant flags for phys_book can be set
    // –> there are no flags implemented at this time
    phys_book.flags = 0;

    // pass virt_book to the architecture in case that becomes useful later
    arch_paging.arch_paging_init(virt_book)
}


/// Allocate the requested number of pages for the given size
///
/// # Arguments
///
/// * `size` - the requested size of the memory to allocate
/// * `hierarchical` - bool saying whether to use a hierarchical allocation system or not
///
/// # Returns
///
/// * `Ok(PhysAddress)` - physical address of the allocated memory if successful
/// * `Err(u8)` - error code if the allocation failed
pub fn alloc(size: usize, hierarchical: bool) -> Result<PhysAddress, u8> {
    // create necessary variables
    let mut hypervisor_struct = HYPERVISOR.lock().unwrap();
    let paging_structs = &mut hypervisor_struct.rust_hypervisor_paging;
    let phys_book = &mut paging_structs.phys_book;
    let arch_paging = &mut paging_structs.arch_paging_struct;
    let virt_book = &mut paging_structs.virt_book;
    let page_size = PAGE_SIZE.load(Ordering::SeqCst);

    let num_pages_needed = num_pages(size, page_size);
    let (chapter_addrs, num_pages): (Vec<PhysAddress>, Vec<NumPages>);

    // if hierarchical is set, use the function that can find multiple chapters
    if hierarchical {
        (chapter_addrs, num_pages) = find_chapters(num_pages_needed, phys_book)?;
    } else {
        (chapter_addrs, num_pages)  = find_chapter(num_pages_needed, phys_book)?;
    }

    // do everything necessary to place the chapter(s) in the bookkeeping structs
    place_chapters(&chapter_addrs, &num_pages, num_pages_needed, phys_book);

    // give the physical address(es) to the architecture to get the virtual address mapping
    let result = arch_paging.create_mapping(&chapter_addrs, num_pages_needed);

    // add the mapping to the mapping structures in the virtual book
    if result.is_ok() {
        let virt_addr = result?;
        for phys_addr in chapter_addrs.iter() {
            virt_book.phys_page_mapping.insert(*phys_addr, (num_pages_needed, virt_addr));
        }
        virt_book.virt_page_mapping.insert(virt_addr, (num_pages_needed, chapter_addrs));
    }

    // return the result of the mapping
    result
}


/// Free the page(s) at the given virtual address
///
/// # Arguments
///
/// * `virt_addr` - the virtual address of the page to free
///
/// # Returns
///
/// * `Ok(())` - if the free was successful
/// * `Err(u8)` - error code if the free failed
pub fn free(virt_addr: VirtAddress) -> Result<(), u8> {

    // create necessary variables
    let mut hypervisor_struct = HYPERVISOR.lock().unwrap();
    let paging_structs = &mut hypervisor_struct.rust_hypervisor_paging;
    let mut phys_book = &mut paging_structs.phys_book;
    let virt_book = &mut paging_structs.virt_book;

    // check if the virtual address is valid
    let phys_addrs = virt_book.virt_page_mapping.get(&virt_addr);
    if phys_addrs.is_none() {
        return Err(CANNOT_FIND_ADDRESS);
    }

    // get the physical address(es) and ignore number of pages used from that mapping
    let (_, phys_addrs) = phys_addrs.unwrap();

    // loop through the physical addresses and remove them from the used pages and virtual page mapping
    for phys_addr in phys_addrs.iter() {
        let num_pages_opt = phys_book.used_pages.get(&phys_addr);
        if num_pages_opt.is_none() {
            return Err(CANNOT_FIND_ADDRESS);
        }

        let num_pages: usize = *num_pages_opt.unwrap();
        phys_book.used_pages.remove(&phys_addr);
        virt_book.phys_page_mapping.remove(phys_addr);

        coalesce(phys_addr, num_pages, &mut phys_book);
    }

    virt_book.virt_page_mapping.remove(&virt_addr);

    Ok(())
}


/// Combine the pages being freed with the (potential) free pages around them
///   using one of two options to find the previous page which is listed in comments in the code.
///
/// # Arguments
///
/// * `phys_addr` - the physical address of the page being freed
/// * `num_pages` - the number of pages being freed
/// * `phys_book` - the physical book struct to update
fn coalesce(phys_addr: &PhysAddress, num_pages: NumPages, phys_book: &mut PhysBook) {

    // create necessary variables
    let mut is_prev_free: bool = false;
    let mut is_next_free: bool = true;
    let free_pages = &mut phys_book.free_pages;
    let page_size = PAGE_SIZE.load(Ordering::SeqCst);

    let mut new_num_pages: usize = num_pages;
    let mut next_page_size = 0;

    // check if the next page is free, the easier one to check
    let next_page_addr = phys_addr + (num_pages*page_size);
    let next_page_size_opt = &mut free_pages.get(&next_page_addr);
    if next_page_size_opt.is_none() {
        is_next_free = false;
    } else {
        next_page_size = *next_page_size_opt.unwrap();
    }

    // There are two options to find the previous page,
    //   use further testing in the future to determine what is best.
    let mut curr_addr: PhysAddress = *phys_addr;
    // option 1: loop through the free pages to see if the previous page is in there
    for (addr, num_pages) in free_pages.iter() {
        if *addr + (*num_pages * page_size) == *phys_addr {
            is_prev_free = true;
            curr_addr = *addr;
            break;
        }
    }


    // Option 2: loop backwards from the current address to find the previous page in
    //   either the free pages or the used pages to determine if it is free.
    // loop {
    //     if curr_addr < page_size {
    //         is_prev_free = false;
    //         break;
    //     }
    //
    //     curr_addr = curr_addr - page_size;
    //     if free_pages.get(&curr_addr).is_some() {
    //         is_prev_free = true;
    //         break;
    //     }
    //     if phys_book.used_pages.get(&curr_addr).is_some() {
    //         is_prev_free = false;
    //         break;
    //     }
    // }


    // insert and remove the relevant pages from the bookkeeping structs
    if is_prev_free {
        new_num_pages += free_pages.get(&curr_addr).unwrap();
        free_pages.remove(&curr_addr);
    } else {
        curr_addr = *phys_addr;
    }
    if is_next_free {
        new_num_pages += next_page_size;
        free_pages.remove(&next_page_addr);
    }

    free_pages.insert(curr_addr, new_num_pages);
}


/// Place the chapter(s) in the structs in the physical book
///
/// # Arguments
///
/// * `chapters` - the physical addresses of the chapters to place
/// * `num_pages` - the number of pages in each chapter
/// * `num_pages_needed` - the number of pages needed for the whole allocation
/// * `phys_book` - the physical book struct to update
fn place_chapters(chapters: &Vec<PhysAddress>, num_pages: &Vec<NumPages>, num_pages_needed: NumPages, phys_book: &mut PhysBook) {
    // create necessary variables
    let free_pages = &mut phys_book.free_pages;
    let used_pages = &mut phys_book.used_pages;
    let page_size = PAGE_SIZE.load(Ordering::SeqCst);
    let mut curr_num_pages_needed = num_pages_needed;

    // loop through the chapters and place them in the free pages
    let num_chapters = chapters.len();
    for i in 0..num_chapters {

        let num_pages_chap = num_pages[i];
        if num_pages_chap < curr_num_pages_needed {
            curr_num_pages_needed -= num_pages_chap;
        }

        free_pages.remove(&chapters[i]);

        // if there are more pages available than needed in the found chapter,
        //   split the chapter into a used chapter and free chapter
        if num_pages_chap > curr_num_pages_needed {
            let new_addr = chapters[i] + curr_num_pages_needed * page_size;
            free_pages.insert(new_addr, num_pages_chap - curr_num_pages_needed);
            used_pages.insert(chapters[i], curr_num_pages_needed);
        } else {
            used_pages.insert(chapters[i], num_pages_chap);
        }
    }

}

/// Find a single chapter to put the requested pages in without a hierarchical system
///
/// # Arguments
///
/// * `num_pages` - the number of pages being requested
/// * `phys_book` - the physical book struct to search
///
/// # Returns
///
/// * `Ok((Vec<PhysAddress>, Vec<NumPages>))` - the physical address and number of pages in the chapter
/// * `Err(u8)` - error code if there wasn't enough memory
fn find_chapter(num_pages: NumPages, phys_book: &mut PhysBook) -> Result<(Vec<PhysAddress>, Vec<NumPages>), u8> {
    let free_pages = &phys_book.free_pages;

    // loop through the free pages to find a single chapter that is large enough
    for (addr, pages) in free_pages.iter() {
        if *pages >= num_pages {
             // create and return vectors to correspond with the hierarchical version
            let mut page = Vec::new();
            let mut num_pages_vec = Vec::new();
            page.push(*addr);
            num_pages_vec.push(*pages);
            return Ok((page, num_pages_vec));
        }
    }

    Err(NOT_ENOUGH_MEMORY)
}

/// Find chapters to put the requested pages in using a hierarchical system
///
/// # Arguments
///
/// * `num_pages` - the number of pages being requested
/// * `phys_book` - the physical book struct to search
///
/// # Returns
///
/// * `Ok((Vec<PhysAddress>, Vec<NumPages>))` - the physical addresses and number of pages in the chapters
/// * `Err(u8)` - error code if there wasn't enough memory
fn find_chapters(num_pages: NumPages, phys_book: &mut PhysBook) -> Result<(Vec<PhysAddress>, Vec<NumPages>), u8> {
    let free_pages = &phys_book.free_pages;
    let mut pages = Vec::new();
    let mut num_pages_chap = Vec::new();
    let mut chapter_size: usize = 0;

    // loop through the free pages to find chapters to put together to have room for the requested pages
    for (addr, num_free_pages) in free_pages.iter() {

            pages.push(*addr);
            num_pages_chap.push(*num_free_pages);
            chapter_size += num_free_pages;

            if chapter_size >= num_pages {
                break;
            }
    }

    // if we found enough pages, return the vectors,
    //   else return an error saying there wasn't enough memory
    if chapter_size >= num_pages {
        Ok((pages, num_pages_chap))
    } else {
        Err(NOT_ENOUGH_MEMORY)
    }
}







