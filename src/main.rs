/// main.rs
///
/// author: Nick Cochran
/// email: nickcochran02@gmail.com
///
/// This file contains the main function and the server that runs the hypervisor
/// in its current form.

mod rust_hypervisor;
mod user_space_arch;
mod exec;

use std::io;
use std::io::{stdout, BufRead, BufReader, Write};
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};
use std::fs::File;
use crate::rust_hypervisor::paging::{alloc, free};
use crate::rust_hypervisor::paging::declarations::{num_pages, NOT_ENOUGH_MEMORY};
use crate::rust_hypervisor::setup::{hv_init, HYPERVISOR, PAGE_SIZE};
use crate::rust_hypervisor::paging::declarations::*;


/// Simple main function to initialize the system then call the
/// function to run the hypervisor.
fn main() -> Result<(), u8> {

    hv_init(SINGLE_CPU_ID)?;
    run_server()
}

/// Runs a server in the command line or through an inputted .txt file.
/// This is meant to be the means through which to run the hypervisor
/// while running in User-Space.
///
/// See README.md for information on running the hypervisor.
///
/// ## Returns
/// - `Ok(())` if the server runs successfully.
/// - `Err(u8)` if there is an error in running the server.
fn run_server() -> Result<(), u8> {
    let stdin = io::stdin();
    let args = std::env::args();

    let mut total_alloc_times: Vec<Duration> = Vec::new();
    let mut total_free_times: Vec<Duration> = Vec::new();
    let mut hierarchical = false; // boolean to use hierarchical allocation
    let mut verbose = false; // boolean to print out more information
    let mut benchmark = false; // boolean to print out benchmark timing
    print!("-> ");
    stdout().flush().expect("Error: could not flush stdout.");

    let args_vec: Vec<String> = args.collect();
    let file;
    let mut file_buf_reader = None;
    if args_vec.len() > CMD_LINE_ARGS_NO_FILE {
        let file_path = args_vec[FILE_NAME_IDX].clone();
        file = match File::open(file_path) {
            Ok(file) => { Some(file) }
            Err(_) => {
                eprintln!("Could not read file.  Continuing with program.");
                None
            }
        };
        if file.is_some() {
            file_buf_reader = Some(BufReader::new(file.as_ref().unwrap()));
        }
    }

    loop {
        let mut two_inputs = false;
        let mut three_inputs = false;

        let mut input = String::new();

        // if there is no file buffered reader, read from stdin
        if file_buf_reader.is_none() {
            stdin.lock().read_line(&mut input).expect("Error: could not read line.");
        } else {
            // read from the file buffered reader and if at the end, set the buffered reader to None
            let res = file_buf_reader.as_mut().unwrap().read_line(&mut input);
            print!("{}", input);
            stdout().flush().expect("Error: could not flush stdout.");
            if res.is_ok() && *res.as_ref().unwrap() == 0 {
                file_buf_reader = None;
            } else if res.is_err() {
                return Err(FAILED_TO_READ_LINE);
            }
        }

        // collect the input line into a vector delimited by spaces
        let input_words : Vec<&str> = input.split_whitespace().collect();

        // if there is no input, continue to the next iteration
        if input_words.is_empty() {
            continue;
        }
        // if there are two or three inputs, set the proper boolean values to true
        if input_words.len() >= 2 {
            two_inputs = true;
            three_inputs = if input_words.len() >= 3 { true } else { false };
        }

        let first_input = input_words[FIRST_INPUT_INDEX];

        // if the first "word" is a double slash, treat it as a comment and ignore it (and reprint the prompt)
        if first_input == "//" {
            print!("-> ");
            stdout().flush().expect("Error: could not flush stdout.");
            continue;
        }

        if first_input == "quit" {
            if verbose {
                println!("Exiting program.");
            }
            // print final results if benchmarking is set to true
            if benchmark {
                println!("Printing out final benchmarking information.");
                print_benchmarking(total_alloc_times, total_free_times);
            }
            break;
        }

        else if first_input == "verbose" {
            verbose = !verbose;
            if verbose {
                println!("Verbose mode turned on.");
            } else {
                println!("Verbose mode turned off.");
            }
        }
        else if first_input == "hierarchical" {
            hierarchical = !hierarchical;
            if verbose {
                if hierarchical {
                    println!("Hierarchical mode turned on.");
                } else {
                    println!("Hierarchical mode turned off.");
                }
            }
        }
        else if first_input == "benchmark" {
            benchmark = !benchmark;
            if verbose {
                if benchmark {
                    println!("Benchmarking mode turned on.");
                } else {
                    println!("Benchmarking mode turned off.");
                }
            }
        }
        // all print related commands
        else if first_input == "print" {

            if two_inputs {
                let second_input = input_words[SECOND_INPUT_INDEX];

                if second_input == "help" {
                    print_usage();
                    print_help();
                }
                else if second_input == "phys_pages" {
                    if three_inputs && input_words[THIRD_INPUT_INDEX] == "free" {
                        print_free_pages();
                    }
                    else if three_inputs && input_words[THIRD_INPUT_INDEX] == "used" {
                        print_used_pages();
                    } else if three_inputs && input_words[THIRD_INPUT_INDEX] == "all" {
                        print_phys_pages();
                    } else {
                        print_phys_pages();
                    }
                }
                else if second_input == "virt_pages" {
                    print_virt_pages();
                }
                else if second_input == "pages" {
                    println!("Printing Physical Pages:");
                    print_phys_pages();
                    println!("Printing Virtual Pages:");
                    print_virt_pages();
                }
            } else {
                print_help();
            }

        }
        // If the command line asks for the alloc operation
        else if first_input == "alloc" && two_inputs {
            let start = Instant::now();
            let size = input_words[SECOND_INPUT_INDEX].parse::<usize>().unwrap();

            match alloc(size, hierarchical) {
                Ok(addr) => {
                    if verbose {
                        println!("allocated {} pages for a size of {} starting at address {}.",
                                 num_pages(size, PAGE_SIZE.load(Ordering::SeqCst)), size, addr);
                    }
                },
                Err(e) => {
                    if verbose {
                        println!("failed to allocate pages.");
                    }
                    if e == NOT_ENOUGH_MEMORY {
                        println!("not enough room to allocate this amount.");
                        continue;
                    }
                    return Err(e);
                }
            };

            let op_time = Instant::now().duration_since(start);
            total_alloc_times.push(op_time);
            if benchmark {
                let micros = op_time.as_micros() as u32;
                println!("Alloc ran in {} microseconds and {} nanoseconds",
                         micros, op_time.subsec_nanos() - (micros * 1000));
            }

        }
        // If the command line asks for the free operation
        else if first_input == "free" && two_inputs {
            let start = Instant::now();
            let addr = input_words[SECOND_INPUT_INDEX].parse::<usize>().unwrap();

            match free(addr) {
                Ok(()) => {
                    if verbose {
                        println!("freed pages for address {}.", addr);
                    }
                },
                Err(e) => {
                    if verbose {
                        println!("failed to free pages.");
                    }
                    return Err(e);
                }
            };

            let op_time = Instant::now().duration_since(start);
            total_free_times.push(op_time);
            if benchmark {
                let micros = op_time.as_micros() as u32;
                println!("Free ran in {} microseconds and {} nanoseconds",
                         micros, op_time.subsec_nanos() - (micros * 1000));
            }

        }
        // Error case for alloc and free
        else if (first_input == "alloc" || first_input == "free") && !two_inputs {
            if verbose {
                println!("Input Error: Must provide another input for alloc or free.");
            }
        }
        // Error default case
        else {
            if verbose {
                println!("Invalid input, try again.");
            }
            print_help();
        }
        print!("-> ");
        stdout().flush().expect("Error: could not flush stdout.");
    }

    Ok(())
}


/// Prints out a message detailing how to run the system and all available commands.
fn print_help() {
    println!("Usage: <command> [<args>]");
    println!("Commands:");
    println!("  alloc <size> - allocate memory of the given size");
    println!("  free <address> - free memory at the given address");
    println!("  print help - print information on running the file and the print command");
    println!("  print <pages> [free|used|all] - print free, used, or all pages");
    println!("  verbose - turn on verbose mode");
    println!("  hierarchical - turn on hierarchical mode");
    println!("  benchmark - turn on benchmarking mode");
    println!("  quit - exit the program");
}


/// Prints out a message detailing how to use the print command.
fn print_usage() {
    println!("Usage of print: print <argument> [<argument>]");
    println!("Arguments:");
    println!("  phys_pages - print physical pages, with additional arguments:");
    println!("      free - print free physical pages");
    println!("      used - print used physical pages");
    println!("      all - print all physical pages");
    println!("  virt_pages - print virtual pages");
    println!("  pages - print all pages");
    println!("  help - print this help message");
}


/// Prints out the time statistics on the operations that have been run.
///
/// ## Parameters
///
/// - `alloc_times`: A vector of type Duration that holds the times of all alloc operations.
/// - `free_times`: A vector of type Duration that holds the times of all free operations.
fn print_benchmarking(alloc_times: Vec<Duration>, free_times: Vec<Duration>) {
    if alloc_times.is_empty() && free_times.is_empty() {
        return;
    }

    let mut alloc_max = &alloc_times[0]; let mut free_max = &free_times[0];
    let mut op_max = if alloc_max > free_max { alloc_max } else { free_max };
    let mut alloc_min = &alloc_times[0]; let mut free_min = &free_times[0];
    let mut op_min = if alloc_min < free_min { alloc_min } else { free_min };

    let mut op_sum = alloc_times[0] + free_times[0];
    let mut alloc_sum = alloc_times[0];
    let mut free_sum = free_times[0];

    for a_time in &alloc_times[1..] {
        if a_time > alloc_max {
            alloc_max = a_time;
            if a_time > op_max {
                op_max = a_time;
            }
        }
        if a_time < alloc_min {
            alloc_min = a_time;
            if a_time < op_min {
                op_min = a_time;
            }
        }
        op_sum += *a_time;
        alloc_sum += *a_time;
    }

    for f_time in &free_times[1..] {
        if f_time > free_max {
            free_max = f_time;
            if f_time > op_max {
                op_max = f_time;
            }
        }
        if f_time < free_min {
            free_min = f_time;
            if f_time < op_min {
                op_min = f_time;
            }
        }
        op_sum += *f_time;
        free_sum += *f_time;
    }

    let alloc_avg = alloc_sum / (alloc_times.len() as u32);
    let free_avg = free_sum / (free_times.len() as u32);
    let op_avg = op_sum / ((alloc_times.len() + free_times.len()) as u32);

    println!("Alloc Time Stats:");
    println!("  Min: {} microseconds and {} nanoseconds",
             alloc_min.as_micros(), alloc_min.subsec_nanos() - ((alloc_min.as_micros() as u32) * 1000));
    println!("  Avg: {} microseconds and {} nanoseconds",
             alloc_avg.as_micros(), alloc_avg.subsec_nanos() - ((alloc_avg.as_micros() as u32) * 1000));
    println!("  Max: {} microseconds and {} nanoseconds",
             alloc_max.as_micros(), alloc_max.subsec_nanos() - ((alloc_max.as_micros() as u32) * 1000));
    println!("Free Time Stats:");
    println!("  Min: {} microseconds and {} nanoseconds",
             free_min.as_micros(), free_min.subsec_nanos() - ((free_min.as_micros() as u32) * 1000));
    println!("  Avg: {} microseconds and {} nanoseconds",
             free_avg.as_micros(), free_avg.subsec_nanos() - ((free_avg.as_micros() as u32) * 1000));
    println!("  Max: {} microseconds and {} nanoseconds",
             free_max.as_micros(), free_max.subsec_nanos() - ((free_max.as_micros() as u32) * 1000));
    println!("Overall Time Stats:");
    println!("  Min: {} microseconds and {} nanoseconds",
             op_min.as_micros(), op_min.subsec_nanos() - ((op_min.as_micros() as u32) * 1000));
    println!("  Avg: {} microseconds and {} nanoseconds",
             op_avg.as_micros(), op_avg.subsec_nanos() - ((op_avg.as_micros() as u32) * 1000));
    println!("  Max: {} microseconds and {} nanoseconds",
             op_max.as_micros(), op_max.subsec_nanos() - ((op_max.as_micros() as u32) * 1000));


}


/// Prints out a list of the physical pages that are available.
fn print_free_pages() {

    let mut hypervisor_struct = HYPERVISOR.lock().unwrap();
    let phys_book = &mut hypervisor_struct.rust_hypervisor_paging.phys_book;
    let free_pages = &mut phys_book.free_pages;
    let page_size = PAGE_SIZE.load(Ordering::SeqCst);

    for (addr, num_pages) in free_pages.iter() {
        println!("{} Free pages in address space {} to {}",
                     num_pages, addr, addr + (num_pages * page_size) - 1);
    }
}


/// Prints out a list of the physical pages that are currently being used.
fn print_used_pages() {

    let mut hypervisor_struct = HYPERVISOR.lock().unwrap();
    let phys_book = &mut hypervisor_struct.rust_hypervisor_paging.phys_book;
    let used_pages = &mut phys_book.used_pages;
    let page_size = PAGE_SIZE.load(Ordering::SeqCst);

    for (addr, num_pages) in used_pages.iter() {
            println!("{} Free pages in address space {} to {}",
                     num_pages, addr, addr + (num_pages * page_size) - 1);
    }
}


/// Prints out a list of all physical pages, both available and used, in order by address.
fn print_phys_pages() {
    let mut hypervisor_struct = HYPERVISOR.lock().unwrap();

    let phys_book = &mut hypervisor_struct.rust_hypervisor_paging.phys_book;
    let free_pages = &mut phys_book.free_pages;
    let used_pages = &mut phys_book.used_pages;
    let page_size = PAGE_SIZE.load(Ordering::SeqCst);

    let mut free_iter = free_pages.iter();
    let mut used_iter = used_pages.iter();
    let mut free_entry = free_iter.next();
    let mut used_entry = used_iter.next();
    loop {
        // if both are None, we are done
        if free_entry.is_none() && used_entry.is_none() {
            break;
        }
        // if there are no free pages left, print the rest of the used pages
        else if free_entry.is_none() {
            let (used_addr, used_num_pages) = used_entry.unwrap();
            println!("{} Used pages in address space {} to {}",
                     used_num_pages, used_addr, used_addr + (used_num_pages * page_size) - 1);
            used_entry = used_iter.next();
        }
        // if there are no used pages left, print the rest of the free pages
        else if used_entry.is_none() {
            let (free_addr, free_num_pages) = free_entry.unwrap();
            println!("{} Free pages in address space {} to {}",
                     free_num_pages, free_addr, free_addr + (free_num_pages * page_size) - 1);
            free_entry = free_iter.next();
        }
        // if both are present, print the one with the lower address
        else {
            let (free_addr, free_num_pages) = free_entry.unwrap();
            let (used_addr, used_num_pages) = used_entry.unwrap();

            if free_addr < used_addr {
                println!("{} Free pages in address space {} to {}",
                         free_num_pages, free_addr, free_addr + (free_num_pages * page_size) - 1);
                free_entry = free_iter.next();
            } else {
                println!("{} Used pages in address space {} to {}",
                         used_num_pages, used_addr, used_addr + (used_num_pages * page_size) - 1);
                used_entry = used_iter.next();
            }
        }
    }
}


/// Prints a list of the virtual pages currently being used.
fn print_virt_pages() {
    let mut hypervisor_struct = HYPERVISOR.lock().unwrap();
    let virt_book = &mut hypervisor_struct.rust_hypervisor_paging.virt_book;
    let virt_pages = &mut virt_book.virt_page_mapping;

    for (virt_addr, (num_pages, _)) in virt_pages.iter() {
        println!("{} Virtual pages in address space {} to {}",
                 num_pages, virt_addr, virt_addr + (num_pages * PAGE_SIZE.load(Ordering::SeqCst)) - 1);
    }
}