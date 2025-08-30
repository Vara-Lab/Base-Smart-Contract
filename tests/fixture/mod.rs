use sails_rs::{
    events::Listener, gtest::{calls::*, System}, prelude::*
};
use contract_client::{
    ContractFactory,
    ContractService,
    contract_service::{
        self, 
        events::ContractServiceEvents
    }
};

#[cfg(debug_assertions)]
pub(crate) const CONTRACT_WASM_PATH: &str = "../../../target/wasm32-gear/debug/contract.opt.wasm";
#[cfg(not(debug_assertions))]
pub(crate) const CONTRACT_WASM_PATH: &str = "../../../target/wasm32-gear/release/contract.opt.wasm";

pub(crate) const ADMIN_ID: u64 = 10;

pub(crate) struct Fixture {
    program_space: GTestRemoting,
    code_id: CodeId
}

impl Fixture {
    pub(crate) fn new() -> Self {
        let system = System::new();
        // system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
        system.init_logger();
        system.mint_to(ADMIN_ID, 1_000_000_000_000_000);

        let code_id = system.submit_code_file(CONTRACT_WASM_PATH);

        let program_space = GTestRemoting::new(system, ADMIN_ID.into());

        Self {
            program_space,
            code_id,
        }
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
