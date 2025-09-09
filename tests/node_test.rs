use contract_client::{contract_service::events::ContractServiceEvents, traits::ContractService};
use gclient::Result;
use sails_rs::{calls::{Action, Call, Query}, events::Listener, futures::StreamExt};
use fixture_node::*;

mod fixture_node;
mod utils;

#[tokio::test]
#[ignore]
async fn test_contract() -> Result<()> {
    let fixture_node = FixtureNode::new().await?;

    let mut contract_client = fixture_node.contract_service_client();
    let mut contract_listener = fixture_node.contract_service_listener();
    let mut contract_events = contract_listener
        .listen()
        .await
        .unwrap();

    // Act - Hello

    let result = contract_client
        .hello()
        .send_recv(fixture_node.contract_address())
        .await
        .unwrap();

    assert_eq!(result, format!("Hello {:?}", fixture_node.api_signer()));

    let event = contract_events
        .next()
        .await
        .unwrap();

    assert_eq!((fixture_node.contract_address(), ContractServiceEvents::Hello(fixture_node.api_signer())), event);

    // Act - Send value

    let result = contract_client
        .send_value()
        .with_value(utils::ONE_TOKEN)
        .send_recv(fixture_node.contract_address())
        .await
        .unwrap();

    assert_eq!(result, "Value get: 1000000000000");

    let event = contract_events
        .next()
        .await
        .unwrap();

    assert_eq!((fixture_node.contract_address(), ContractServiceEvents::ValueReceived(utils::ONE_TOKEN)), event);

    let contract_balance = fixture_node.balance_of(fixture_node.contract_address()).await;

    assert_eq!(utils::ONE_TOKEN * 2, contract_balance);

    // Act - Get value

    let result = contract_client
        .get_value(utils::ONE_TOKEN)
        .send_recv(fixture_node.contract_address())
        .await
        .unwrap();

    assert_eq!(result, format!("Value returned: {}", utils::ONE_TOKEN));

    let event = contract_events
        .next()
        .await
        .unwrap();

    assert_eq!((fixture_node.contract_address(), ContractServiceEvents::ValueSent(utils::ONE_TOKEN)), event);

    let contract_balance = fixture_node.balance_of(fixture_node.contract_address()).await;

    assert_eq!(contract_balance, utils::ONE_TOKEN);

    // Act - Increment value

    let result = contract_client
        .increment()
        .send_recv(fixture_node.contract_address())
        .await
        .unwrap();

    assert_eq!(result, 1);

    let event = contract_events
        .next()
        .await
        .unwrap();

    assert_eq!((fixture_node.contract_address(), ContractServiceEvents::Incremented), event);

    // Act - Get current counter value

    let counter_value = contract_client
        .counter_value()
        .recv(fixture_node.contract_address())
        .await
        .unwrap();

    assert_eq!(counter_value, 1);

    // Act - Decrement value

    let result = contract_client
        .decrement()
        .send_recv(fixture_node.contract_address())
        .await
        .unwrap();

    assert_eq!(result, 0);

    let event = contract_events
        .next()
        .await
        .unwrap();

    assert_eq!((fixture_node.contract_address(), ContractServiceEvents::Decremented), event);

    // Act error - Decrement value

    let result = contract_client
        .decrement()
        .send_recv(fixture_node.contract_address())
        .await;

    assert!(result.is_err());

    // Act error - Get value error

    let result = contract_client
        .get_value(utils::ONE_TOKEN)
        .send_recv(fixture_node.contract_address())
        .await;

    assert!(result.is_err());

    Ok(())
}