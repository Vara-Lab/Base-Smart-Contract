#![no_std]

use gstd::msg;
// necesary modules
use sails_rs::{
    prelude::*,
    cell::RefCell
};

// import into scope contract modules
pub mod services;
pub mod state;
pub mod utils;

// Import the contract service and the state
use services::contract_service::ContractService;
use state::contract_state::ContractState;

// ContractProgram struct to build the program ((there can only be one per contract))
pub struct ContractProgram {
    state: RefCell<ContractState>,
}

// ContractProgram: program, it host one or more services and it expose them to the 
// externar consumer.
// Only one program is allowed per application
#[program]
impl ContractProgram {
    // Application constructor (it is an associated function)
    // It can be called once per application lifetime.
    pub fn new() -> Self {
        let mut contract_state = ContractState::default();
        contract_state.owner = msg::source();

        Self {
            state: RefCell::new(contract_state),
        }
    }

    // Method working with "&self", having no other parameters are treated as exposed
    // service constructors, and are called each time when an incoming request message 
    // needs be dispatched to a selected service
    // It has "message routing", This will change the way a service will be called 
    // (if omitted, the method name will be used, in this case ContractSvc).
    #[export(route = "TrafficLight")]
    pub fn contract_svc(&self) -> ContractService<'_> {
        ContractService::new(&self.state)
    }
}