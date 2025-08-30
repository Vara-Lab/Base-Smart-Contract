use contract_client::{
    traits::*,
    contract_factory,
    contract_service::events::ContractServiceEvents
};
use fixture::{
    ADMIN_ID,
    CONTRACT_WASM_PATH,
    Fixture
};
use gstd::{errors::{
    ErrorReplyReason, 
    SimpleExecutionError
}, ActorId};
use sails_rs::{
    calls::*,
    errors::RtlError,
    events::*,
    futures::StreamExt,
    gtest::{
        Program, System,
        calls::{BlockRunMode, GTestRemoting},
    },
};

mod fixture;

#[tokio::test]
async fn hello_world() {
    // Arrange

    let fixture = Fixture::new();
    let factory = fixture.contract_factory();
    let contract_id = factory
        .new()
        .send_recv(fixture.contract_code_id(), "123")
        .await
        .unwrap();

    let mut contract_client = fixture.contract_service_client();

    // Listen to contract service events
    let mut contract_listener = fixture.contract_service_listener();
    let mut contract_events = contract_listener
        .listen()
        .await
        .unwrap();

    // Act

    // Using generated client code for calling Contract service
    // using the `send_recv` method
    let result = contract_client
        .hello()
        .send_recv(contract_id)
        .await
        .unwrap();

    // Assert

    let event = contract_events
        .next()
        .await
        .unwrap();

    assert_eq!(result, format!("Hello {:?}", ActorId::zero()));
    assert_eq!((contract_id, ContractServiceEvents::Hello(ActorId::zero())), event)

}

