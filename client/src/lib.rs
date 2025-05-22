#![no_std]

// Incorporate the generated code based on the idl file
include!(concat!(env!("OUT_DIR"), "/contract_client.rs"));
