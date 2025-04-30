# RustHypervisor

### Author: Nick Cochran
### Email: nickcochran02@gmail.com
### Advisor: Prof. Chris Gill

This repository contains a hypervisor written in Rust, as a part of a research project
under the larger project umbrella of creating a Cyber-Physical Operating System (CPOS)
in Rust.  The current iteration is a prototype that runs in user-space and is meant
mainly for testing purposes for future iterations.

## Running the Hypervisor

The hypervisor currently runs in the command line using commands such as
`alloc` and `free`.  Further information on the commands available can be found
by running `print help` in the command line with the program running.

## Testing

Testing was accomplished by running the program and ensuring accuracy manually,
and then using a python script and pre-built files to further test the code.