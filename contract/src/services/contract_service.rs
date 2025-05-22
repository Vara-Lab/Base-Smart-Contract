use sails_rs::{
    prelude::*,
    cell::RefCell,
    gstd::msg
};

// Import the state struct
use crate::state::contract_state::{
    ContractState,
    IoContractState,
    Light
};
use crate::utils;

// ContractService struct to build the service
pub struct ContractService<'a> {
    state: &'a RefCell<ContractState>,
}

// Service 
#[service]
impl<'a> ContractService<'a> {
    // Service constructor
    pub fn new(
        state: &'a RefCell<ContractState>,
    ) -> Self {
        Self { state }
    }

    // Remote call "green" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    pub fn green(&mut self) -> ContractResponse {
        // panicking function: By using errors, smart contract implementation becomes cheaper.
        // In case of any error, it will capture it and notify you 
        // along with an extra payload (which derives from debug, as in this case, String),
        // and in case of success, it will return the response.
        // You need to specify the types to return, in this case:
        // - In case os success, it returns ContractResponse.
        // - In case of error, it will set a String in the error payload.
        // - You can omit the third type (with _).
        utils::panicking::<ContractResponse, String, _>(|| {
            // Get state as mut
            let mut state = self
                .state
                .try_borrow_mut() // Try to get the mutable state
                // if error, map the error to String, and throw the error with "?"
                .map_err(|error| error.to_string())?; 

            // Changing state
            state.current_light = Light::Green;
            state.callers.insert(msg::source(), Light::Green);

            // returning the response
            Ok(ContractResponse::GreenReceived)
        })
    }

    // Remote call "yellow" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    pub fn yellow(&mut self) -> ContractResponse {
        // panicking function: By using errors, smart contract implementation becomes cheaper.
        // In case of any error, it will capture it and notify you 
        // along with an extra payload (which derives from debug, as in this case, String),
        // and in case of success, it will return the response.
        // You need to specify the types to return, in this case:
        // - In case os success, it returns ContractResponse.
        // - In case of error, it will set a String in the error payload.
        // - You can omit the third type (with _).
        utils::panicking::<ContractResponse, String, _>(|| {
            // Get state as mut
            let mut state = self
                .state
                .try_borrow_mut() // Try to get the mutable state
                // if error, map the error to String, and throw the error with "?"
                .map_err(|error| error.to_string())?;

            // Changing state
            state.current_light = Light::Yellow;
            state.callers.insert(msg::source(), Light::Yellow);

            // returning the response
            Ok(ContractResponse::YellowReceived)
        })
    }

    // Remote call "yellow" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    pub fn red(&mut self) -> ContractResponse {
        // panicking function: By using errors, smart contract implementation becomes cheaper.
        // In case of any error, it will capture it and notify you 
        // along with an extra payload (which derives from debug, as in this case, String),
        // and in case of success, it will return the response.
        // You need to specify the types to return, in this case:
        // - In case os success, it returns ContractResponse.
        // - In case of error, it will set a String in the error payload.
        // - You can omit the third type (with _).
        utils::panicking::<ContractResponse, String, _>(|| {
            // Get state as mut
            let mut state = self
                .state
                .try_borrow_mut() // Try to get the mutable state
                // if error, map the error to String, and throw the error with "?"
                .map_err(|error| error.to_string())?;

            // Changing state
            state.current_light = Light::Red;
            state.callers.insert(msg::source(), Light::Red);

            // returning the response
            Ok(ContractResponse::RedReceived)
        })
    }

    // Remote call "contract_owner" exposed to external consumers
    // Returns the contracts owner actor id
    // Is treated as a query, keeping everything unchanged and returning some data. (&self)
    pub fn contract_owner(&self) -> ActorId {
        let state = self.state.borrow();

        state.owner
    }

    // Remote call "traffic_light" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a query, keeping everything unchanged and returning some data. (&self)
    pub fn traffic_light(&self) -> IoContractState {
        let state = self.state.borrow();
        state.clone()
            .into()
    }
}

// struct to use as a response to the user
#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ContractResponse {
    GreenReceived,
    YellowReceived,
    RedReceived,
}