// use contract_client::{
//     traits::*,
//     contract_service::events::ContractServiceEvents
// };

use contract::client::{
    traits::*,
    contract_service::events::ContractServiceEvents
};
use fixture::{
    ADMIN_ID,
    Fixture
};
use sails_rs::{
    calls::*,
    events::*,
    futures::StreamExt,
};
use gstd::ActorId;

mod fixture;
mod utils;

#[tokio::test]
async fn hello_world() {
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

    assert_eq!(result, format!("Hello {:?}", ActorId::from(ADMIN_ID)));
    assert_eq!((contract_id, ContractServiceEvents::Hello(ActorId::from(ADMIN_ID))), event)

}

#[tokio::test]
async fn increment_and_decrement() {
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

    // Assert increment

    let result = contract_client
        .increment()
        .send_recv(contract_id)
        .await
        .unwrap();

    assert_eq!(result, 1);
    
    let event = contract_events
        .next()
        .await
        .unwrap();

    assert_eq!((contract_id, ContractServiceEvents::Incremented), event);

    // Assert value

    let result = contract_client
        .counter_value()
        .recv(contract_id)
        .await
        .unwrap();

    assert_eq!(result, 1);

    // Assert decrement

    let result = contract_client
        .decrement()
        .send_recv(contract_id)
        .await
        .unwrap();

    assert_eq!(result, 0);

    let event = contract_events
        .next()
        .await
        .unwrap();

    assert_eq!((contract_id, ContractServiceEvents::Decremented), event);

    // Assert error - decrement value

    let result = contract_client
        .decrement()
        .send_recv(contract_id)
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn send_and_get_value() {
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

    // Assert send value

    let result = contract_client
        .send_value()
        .with_value(utils::ONE_TOKEN)
        .send_recv(contract_id)
        .await
        .unwrap();

    assert_eq!(result, format!("Value get: {}", utils::ONE_TOKEN));

    let contract_balance = fixture.balance_of(contract_id);

    assert_eq!(contract_balance, utils::ONE_TOKEN * 2);

    let event = contract_events
        .next()
        .await
        .unwrap();

    assert_eq!((contract_id, ContractServiceEvents::ValueReceived(utils::ONE_TOKEN)), event);

    // Assert get value

    let result = contract_client
        .get_value(utils::ONE_TOKEN)
        .send_recv(contract_id)
        .await
        .unwrap();

    assert_eq!(result, format!("Value returned: {}", utils::ONE_TOKEN));

    let contract_balance = fixture.balance_of(contract_id);

    assert_eq!(contract_balance, utils::ONE_TOKEN);

    let event = contract_events
        .next()
        .await
        .unwrap();

    assert_eq!((contract_id, ContractServiceEvents::ValueSent(utils::ONE_TOKEN)), event);

    // Assert error - Get balance

    let result = contract_client
        .get_value(utils::ONE_TOKEN)
        .send_recv(contract_id)
        .await;

    assert!(result.is_err());

}