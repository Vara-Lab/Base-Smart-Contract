#![no_std]

use sails_rs::{
    prelude::*,
    cell::RefCell,
};

pub mod services;

use services::contract_service::{ContractService, CounterData};

pub struct ContractProgram {
    state: RefCell<CounterData>,
}

// Program contains "payable" argument because it will receive tokens
#[program(payable)]
impl ContractProgram {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(Default::default()),
        }
    }

    #[export(route = "ContractService")]
    pub fn contract_svc(&self) -> ContractService<'_> {
        ContractService::new(&self.state)
    }
}

// Mock `Syscall` to simulate the environment
// Test service
#[cfg(test)]
mod tests {
    use sails_rs::gstd::services::Exposure;
    use super::*;

    #[tokio::test]
    async fn contract_service_exposure() {
        let message_source = ActorId::from(3);
        let program = ContractProgram::new();

        Syscall::with_message_source(message_source);

        let mut service_exposure = program.contract_svc();

        let response = service_exposure.hello();

        // Assert

        assert_eq!("ContractService".encode().as_slice(), service_exposure.route());
        assert_eq!(format!("Hello {:?}", message_source), response);
    }
}