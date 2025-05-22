#[cfg(test)]
mod tests {
    use sails_rs::calls::*;
    use client::{
        traits::{
            ContractFactory,
            TrafficLight,
        },
        ContractFactory as Factory,
        TrafficLight as ContractTrafficLightClient,
    };
    use crate::utils::utils;

    #[tokio::test]
    pub async fn test_green() {
        let (program_space, code_id) = utils::program_space_and_code_id(
            utils::ADMIN_ID, // admin actor id
            vec![ // actors id to mint the specified tokens
                utils::ADMIN_ID, 
            ], 
            100 // 100 Varas
        );

        let contract_factory = Factory::new(program_space.clone());
        let contract_id = contract_factory
            .new()
            .send_recv(code_id, "123")
            .await
            .unwrap();

        let mut client = ContractTrafficLightClient::new(program_space);

        // Call green method

        let response = client
            .green()
            .send_recv(contract_id)
            .await
            .unwrap();

        assert_eq!(response, client::ContractResponse::GreenReceived);
    }

    #[tokio::test]
    pub async fn test_yellow() {
        let (program_space, code_id) = utils::program_space_and_code_id(
            utils::ADMIN_ID, // admin actor id
            vec![ // actors id to mint the specified tokens
                utils::ADMIN_ID, 
            ], 
            100 // 100 Varas
        );

        let contract_factory = Factory::new(program_space.clone());
        let contract_id = contract_factory
            .new()
            .send_recv(code_id, "123")
            .await
            .unwrap();

        let mut client = ContractTrafficLightClient::new(program_space);

        // Call yellow method

        let response = client
            .yellow()
            .send_recv(contract_id)
            .await
            .unwrap();

        assert_eq!(response, client::ContractResponse::YellowReceived);
    }

    #[tokio::test]
    pub async fn test_red() {
        let (program_space, code_id) = utils::program_space_and_code_id(
            utils::ADMIN_ID, // admin actor id
            vec![ // actors id to mint the specified tokens
                utils::ADMIN_ID, 
            ], 
            100 // 100 Varas
        );

        let contract_factory = Factory::new(program_space.clone());
        let contract_id = contract_factory
            .new()
            .send_recv(code_id, "123")
            .await
            .unwrap();

        let mut client = ContractTrafficLightClient::new(program_space);

        // Call red method

        let response = client
            .red()
            .send_recv(contract_id)
            .await
            .unwrap();

        assert_eq!(response, client::ContractResponse::RedReceived);
    }
}
