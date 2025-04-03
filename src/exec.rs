/// REPLICA OF CPOS FILE FOR TEMP USE


/// Module: cpos::exec
/// File: exec.rs
/// Author: Chris Gill
/// Purpose: Declare and implement empty base execution context descriptor struct

// To use error codes for this module, add a line like the following
// use super::error::ERR_BAD_CONTROL;

#[derive(Debug,Clone)]
pub struct EcdBase{}
impl EcdBase {pub fn new() -> EcdBase {EcdBase {}}}