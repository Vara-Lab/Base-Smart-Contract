// necesary modules
use sails_rs::{
    prelude::*,
    collections::HashMap,
};

// enum for each state of the traffic light
#[derive(Default, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Light {
    #[default]
    Green,
    Red,
    Yellow
}

// Create a struct for the state
#[derive(Clone, Default)]
pub struct ContractState {
    pub owner: ActorId,
    pub current_light: Light,
    pub callers: HashMap<ActorId, Light>,
}

// Create a struct that can be send to the user who reads state
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoContractState {
    pub owner: ActorId,
    pub current_light: Light,
    pub callers: Vec<(ActorId, Light)>,
}

// Implementation of the From trait for converting CustomStruct to IoCustomStruct
impl From<ContractState> for IoContractState {
    // Conversion method
    fn from(value: ContractState) -> Self {
        // Destructure the CustomStruct object into its individual fields
        let ContractState {
            owner,
            current_light,
            callers,
        } = value;

        // Perform some transformation on second field, cloning its elements (Warning: Just for HashMaps!!)
        let callers = callers
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect();
   
        // Create a new IoCustomStruct object using the destructured fields
        Self {
            owner,
            current_light,
            callers,
        }
    }
}
