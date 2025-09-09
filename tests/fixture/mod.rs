use sails_rs::{
    events::Listener, 
    gtest::{calls::*, System}, 
    prelude::*
};
use contract_client::{
    ContractFactory,
    ContractService,
    contract_service::{
        self, 
        events::ContractServiceEvents
    }
};

use contract::WASM_BINARY;

pub(crate) const ADMIN_ID: u64 = 10;
pub(crate) const ONE_TOKEN: u128 = 1_000_000_000_000;

pub(crate) struct Fixture {
    program_space: GTestRemoting,
    code_id: CodeId
}

impl Fixture {
    pub(crate) fn new() -> Self {
        let system = System::new();
        system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
        system.init_logger();
        system.mint_to(ADMIN_ID, ONE_TOKEN * 1_000);

        let code_id = system.submit_code(WASM_BINARY);

        let program_space = GTestRemoting::new(system, ADMIN_ID.into());

        Self {
            program_space,
            code_id,
        }
    }

    pub(crate) fn balance_of(&self, address: ActorId) -> u128 {
        self
            .program_space
            .system()
            .balance_of(address)
    }

    pub(crate) fn contract_code_id(&self) -> CodeId {
        self.code_id
    }

    pub(crate) fn contract_factory(&self) -> ContractFactory<GTestRemoting> {
        ContractFactory::new(self.program_space.clone())
    }

    pub(crate) fn contract_service_client(&self) -> ContractService<GTestRemoting> {
        ContractService::new(self.program_space.clone())
    }

    pub(crate) fn contract_service_listener(&self) -> impl Listener<ContractServiceEvents> {
        contract_service::events::listener(self.program_space.clone())
    }
}
