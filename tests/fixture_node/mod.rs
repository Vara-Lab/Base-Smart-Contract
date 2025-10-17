#![allow(unused)]
use gclient::{EventProcessor, GearApi, Result};
use sails_rs::{events::Listener, gclient::calls::GClientRemoting, ActorId, Encode};
use gstd::MessageId;

use contract::client::{
    ContractFactory,
    ContractService, // client for this service
    contract_service::{
        self,
        events::ContractServiceEvents
    }
};

use contract::WASM_BINARY;

pub trait ApiUtils {
    fn get_actor_id(&self) -> ActorId;
    fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId;
    async fn total_balance_of(&self, address: ActorId) -> u128;
} 

pub(crate) struct FixtureNode {
    program_space: GClientRemoting,
    contract_address: ActorId,
    api: GearApi,
}

impl FixtureNode {
    pub(crate) async  fn new() -> Result<Self> {
        // Connect to testnet instead of create a local node 
        // you can test your contract in a local node followint the next steps:
        // - Download the binary in https://get.gear.rs
        // - Set in your code: let api = GearApi::dev_from_path("../target/tmp/gear").await?;
        //   where the path is where your binary is located
        let api = GearApi::vara_testnet().await?;
        let mut listener = api.subscribe().await?;

        assert!(listener.blocks_running().await?);

        let (message_id, program_id) = init(&api).await;

        assert!(listener.message_processed(message_id).await?.succeed());

        let program_space = GClientRemoting::new(api.clone());

        Ok(Self { program_space, contract_address: program_id, api })
    }

    pub(crate) async fn new_client_with_tokwns(&self, name: &str, tokens: u128) -> Self {
        let new_api = get_new_client_with_tokens(&self.api, name, tokens).await;
        let program_space = GClientRemoting::new(new_api.clone());

        Self {
            program_space,
            contract_address: self.contract_address,
            api: new_api
        }
    }

    pub(crate) async fn balance_of(&self, address: ActorId) -> u128 {
        self.api.total_balance_of(address).await
    }

    pub(crate) fn api_signer(&self) -> ActorId {
        self.api.get_actor_id()
    }

    pub(crate) fn contract_address(&self) -> ActorId {
        self.contract_address
    }

    pub(crate) fn contract_factory(&self) -> ContractFactory<GClientRemoting> {
        ContractFactory::new(self.program_space.clone())
    }

    pub(crate) fn contract_service_client(&self) -> ContractService<GClientRemoting> {
        ContractService::new(self.program_space.clone())
    }

    pub(crate) fn contract_service_listener(&self) -> impl Listener<ContractServiceEvents> {
        contract_service::events::listener(self.program_space.clone()) 
    }
}

impl ApiUtils for GearApi {
    fn get_actor_id(&self) -> ActorId {
        ActorId::new(
            self.account_id()
                .encode()
                .try_into()
                .expect("Unexpected invalid account id length."),
        )
    }
    fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId {
        let api_temp = self
            .clone()
            .with(value)
            .expect("Unable to build `GearApi` instance with provided signer.");
        api_temp.get_actor_id()
    }
    async fn total_balance_of(&self, address: ActorId) -> u128 {
        let balance = self.total_balance(address)
            .await
            .expect("Error total balance");

        balance
    }
}

pub async fn get_new_client_with_tokens(api: &GearApi, name: &str, tokens_amount: u128) -> GearApi {
    let alice_balance = api.total_balance_of(api.get_actor_id()).await;

    if alice_balance <= tokens_amount {
        panic!("Alice balance is not enough: {} < {}", alice_balance, tokens_amount);
    }

    api.transfer_keep_alive(
        api.get_specific_actor_id(name)
            .encode()
            .as_slice()
            .try_into()
            .expect("Unexpected invalid `ActorId`."),
        tokens_amount,
    )
    .await
    .expect("Error transfer");

    api.clone().with(name).expect("Unable to change signer")
}

pub async fn init(api: &GearApi) -> (MessageId, ActorId) {
    let request = ["New".encode(), ().encode()].concat();

    let gas_info = api
        .calculate_upload_gas(
            None,
            WASM_BINARY.into(),
            request.clone(),
            0,
            true,
        )
        .await
        .expect("Error calculate upload gas");

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            WASM_BINARY,
            gclient::now_micros().to_le_bytes(),
            request,
            gas_info.min_limit,
            0,
        )
        .await
        .expect("Error upload program bytes");

    (message_id, program_id)
}