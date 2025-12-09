use sails_rs::{
    futures::StreamExt as _,
    gclient::Result
};
use contract_client::{ContractClient, contract_service::{ContractService, events::ContractServiceEvents}};
use fixture_node::*;

mod fixture_node;
mod utils;

// #[tokio::test]
// #[ignore]
// async fn testing() {
//     let mut fixture_node = FixtureNode::new()
//         .await
//         .unwrap();
//     let _code_id = fixture_node
//         .upload_contract_to_testnet()
//         .await;
//     let contract_program = fixture_node
//         .create_contract()
//         .await;

//     println!("ACTOR ID: {:?}", contract_program.id());
//     println!("CODE ID:  {:?}", fixture_node.contract_code_id());

//     // let t = FixtureNode::new()
//     //     .await
//     //     .unwrap();

//     // t.upload_contract_to_testnet().await;
// }

/*
ACTOR ID: 0xdd97c198028f11e7da7b3ec76ec064713fed632d78e9907d7a3876986043c623
CODE ID:  Some(0x7d6f8efe0df726eb74873bfd8d6a246f5485973134bd29471b4e14f82539f0d2)

ACTOR ID: 0x2e84b90091003072df1e746acd4797e307e7f7404b7372da1d4bb2306df0c441
CODE ID:  Some(0x3dcb186fa78734203158fe5627949726d7f96f7ce6c4e9f8b32ca49d3f2455a7)

ACTOR ID: 0x9e3ca062e4eca1003082c1b497470101c56b51c0119f2cd357de121adf3dfb2f
CODE ID:  Some(0x3dcb186fa78734203158fe5627949726d7f96f7ce6c4e9f8b32ca49d3f2455a7)

ACTOR ID: 0xfed3cea1dc619859ba40c476846a76bc97dd4876a2d0329b14d4f6d4b61036de
CODE ID:  Some(0x3dcb186fa78734203158fe5627949726d7f96f7ce6c4e9f8b32ca49d3f2455a7)
*/








// use contract::client::{contract_service::events::ContractServiceEvents, traits::ContractService};
// use sails_rs::{calls::{Action, Call, Query}, events::Listener, futures::StreamExt};
// use fixture_node::*;

// mod fixture_node;
// mod utils;

#[tokio::test]
#[ignore]
async fn test_contract() -> Result<()> {
    let mut fixture_node = FixtureNode::new().await?;
    let _code_id = fixture_node
        .upload_contract_to_testnet()
        .await;
    let contract_program = fixture_node
        .create_contract()
        .await;

    let mut service_client = contract_program.contract_service();

    let service_listener = service_client.listener();
    let mut service_events = service_listener   
        .listen()
        .await
        .unwrap();


    // let fixture_node = FixtureNode::new().await?;

    // let mut contract_client = fixture_node.contract_service_client();
    // let mut contract_listener = fixture_node.contract_service_listener();
    // let mut contract_events = contract_listener
    //     .listen()
    //     .await
    //     .unwrap();

    // Act - Hello

    // Use generated client code for calling ContractService service.
    // To send a message, you must specify:
    // - Service to send the message
    // - Service method to call
    // Or use the client that you get before.

    let response = service_client
        .hello()
        .await
        .unwrap();

    // let result = contract_client
    //     .hello()
    //     .send_recv(fixture_node.contract_address())
    //     .await
    //     .unwrap();

    assert_eq!(response, format!("Hello {:?}", fixture_node.api_signer()));

    // Get event
    let event = service_events
        .next()
        .await
        .unwrap();

    assert_eq!((contract_program.id(), ContractServiceEvents::Hello(fixture_node.api_signer())), event);

    // let event = contract_events
    //     .next()
    //     .await
    //     .unwrap();

    // assert_eq!((fixture_node.contract_address(), ContractServiceEvents::Hello(fixture_node.api_signer())), event);

    // Act - Send value

    let response = contract_program
        .contract_service()
        .send_value()
        .with_value(utils::ONE_VARA)
        .await
        .unwrap();

    // let result = contract_client
    //     .send_value()
    //     .with_value(utils::ONE_TOKEN)
    //     .send_recv(fixture_node.contract_address())
    //     .await
    //     .unwrap();

    assert_eq!(response, "Value get: 1000000000000");

    // let event = contract_events
    //     .next()
    //     .await
    //     .unwrap();

    let event = service_events
        .next()
        .await
        .unwrap();

    assert_eq!((contract_program.id(), ContractServiceEvents::ValueReceived(utils::ONE_VARA)), event);

    let contract_balance = fixture_node.balance_of(contract_program.id()).await;

    assert_eq!(utils::ONE_VARA * 2, contract_balance);

    // Act - Get value

    let response = contract_program
        .contract_service()
        .get_value(utils::ONE_VARA)
        .await
        .unwrap();

    // let result = contract_client
    //     .get_value(utils::ONE_TOKEN)
    //     .send_recv(fixture_node.contract_address())
    //     .await
    //     .unwrap();

    assert_eq!(response, format!("Value returned: {}", utils::ONE_VARA));

    // let event = contract_events
    //     .next()
    //     .await
    //     .unwrap();

    let event = service_events
        .next()
        .await
        .unwrap();

    assert_eq!((contract_program.id(), ContractServiceEvents::ValueSent(utils::ONE_VARA)), event);

    let contract_balance = fixture_node.balance_of(contract_program.id()).await;

    assert_eq!(contract_balance, utils::ONE_VARA);

    // Act - Increment value

    let response = contract_program
        .contract_service()
        .increment()
        .await
        .unwrap();
        
    // let result = contract_client
    //     .increment()
    //     .send_recv(fixture_node.contract_address())
    //     .await
    //     .unwrap();

    assert_eq!(response, 1);

    // let event = contract_events
    //     .next()
    //     .await
    //     .unwrap();

    let event = service_events
        .next()
        .await
        .unwrap();

    assert_eq!((contract_program.id(), ContractServiceEvents::Incremented), event);

    // Act - Get current counter value

    let counter_value = contract_program    
        .contract_service()
        .counter_value()
        .await
        .unwrap();

    // let counter_value = contract_client
    //     .counter_value()
    //     .recv(fixture_node.contract_address())
    //     .await
    //     .unwrap();

    assert_eq!(counter_value, 1);

    // Act - Decrement value

    let response = contract_program
        .contract_service()
        .decrement()
        .await
        .unwrap();

    // let result = contract_client
    //     .decrement()
    //     .send_recv(fixture_node.contract_address())
    //     .await
    //     .unwrap();

    assert_eq!(response, 0);

    // let event = contract_events
    //     .next()
    //     .await
    //     .unwrap();

    let event = service_events
        .next()
        .await
        .unwrap();

    assert_eq!((contract_program.id(), ContractServiceEvents::Decremented), event);

    // Act error - Decrement value

    let response = contract_program
        .contract_service()
        .decrement()
        .await;

    // let result = contract_client
    //     .decrement()
    //     .send_recv(fixture_node.contract_address())
    //     .await;

    assert!(response.is_err());

    // Act error - Get value error

    let response = contract_program
        .contract_service()
        .get_value(utils::ONE_VARA)
        .await;

    // let result = contract_client
    //     .get_value(utils::ONE_TOKEN)
    //     .send_recv(fixture_node.contract_address())
    //     .await;

    assert!(response.is_err());

    Ok(())
}