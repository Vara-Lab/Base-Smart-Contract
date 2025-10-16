use sails_rs::{
    prelude::*,
    cell::RefCell
};

#[event]
#[derive(Encode, TypeInfo, Debug)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ContractEvent {
    Hello(ActorId),
    ValueReceived(u128),
    ValueSent(u128),
    Incremented,
    Decremented
}

#[derive(Default)]
pub struct CounterData {
    counter: u64
}

pub struct ContractService<'a> {
    state: &'a RefCell<CounterData>,
}

impl <'a> ContractService<'a> {
    pub fn new(state: &'a RefCell<CounterData>) -> Self {
        Self { state }
    }
}

#[service(events = ContractEvent)]
impl ContractService<'_> {
    #[export]
    pub fn hello(&mut self) -> String {
        let msg_source = Syscall::message_source();

        self.emit_event(ContractEvent::Hello(Syscall::message_source())).unwrap();

        format!("Hello {:?}", msg_source)
    }

    #[export]
    pub fn send_value(&mut self) -> String {
        let value = Syscall::message_value();
        self.emit_event(ContractEvent::ValueReceived(value)).unwrap();

        format!("Value get: {}", value)
    }

    #[export]
    pub fn get_value(&mut self, to_return: u128) -> CommandReply<String> {
        let contract_tokens = Syscall::value_available();

        if contract_tokens >= to_return {
            self.emit_event(ContractEvent::ValueSent(to_return)).unwrap();
            CommandReply::new(format!("Value returned: {}", to_return)).with_value(to_return)
        } else {
            panic!("Cant transfer tokens");
        }
    }

    #[export]
    pub fn increment(&mut self) -> u64 {
        let mut state = self.state.borrow_mut();
                
        self.emit_event(ContractEvent::Incremented).unwrap();

        state.counter += 1;
        state.counter
    }

    #[export(unwrap_result)]
    pub fn decrement(&mut self) -> Result<u64, String> {
        let mut state = self.state.borrow_mut();
        
        state.counter = state.counter
            .checked_sub(1)
            .ok_or("Counter can not be negative!".to_string())?;

        self.emit_event(ContractEvent::Decremented).unwrap();

        Ok(state.counter)
    }

    #[export]
    pub fn counter_value(&self) -> u64 {
        self.state.borrow().counter
    }
}

#[cfg(test)]
mod tests {
    use sails_rs::gstd::services::Service;
    use super::*;

    #[test]
    pub fn test_hello() {
        Syscall::with_message_source(ActorId::from(3));

        let state = RefCell::new(Default::default());
        let mut contract_service = ContractService::new(&state).expose(&[]);

        let response = contract_service.hello();

        let expected_result = format!("Hello {:?}", ActorId::from(3));

        assert_eq!(expected_result, response);
    }

    #[test]
    pub fn test_send_value() {
        Syscall::with_message_value(1000);

        let state = RefCell::new(Default::default());
        let mut contract_service = ContractService::new(&state).expose(&[]);

        let response = contract_service.send_value();

        assert_eq!("Value get: 1000", response);
    }

    #[test]
    pub fn test_get_value() {
        Syscall::with_value_available(10000);

        let state = RefCell::new(Default::default());
        let mut contract_service = ContractService::new(&state).expose(&[]);


        let (response, amount) = contract_service.get_value(1000).to_tuple();

        assert_eq!(amount, 1000);
        assert_eq!(response, "Value returned: 1000");
    }

    #[test]
    pub fn test_increment_value() {
        let state = RefCell::new(Default::default());
        let mut contract_service = ContractService::new(&state).expose(&[]);

        let response = contract_service.increment();

        assert_eq!(response, 1);
        assert_eq!(state.borrow().counter, 1);
    }

    #[test]
    pub fn test_decrement_value() {
        let state = RefCell::new(Default::default());
        let mut contract_service = ContractService::new(&state).expose(&[]);

        let response = contract_service.increment();

        assert_eq!(response, 1);
        assert_eq!(state.borrow().counter, 1);

        let response = contract_service.decrement();

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), 0);
    }

    #[test]
    pub fn test_decrement_error() {
        let state = RefCell::new(Default::default());
        let mut contract_service = ContractService::new(&state).expose(&[]);

        let response = contract_service.decrement();

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), "Counter can not be negative!");
    }

    #[test]
    #[should_panic(expected = "Cant transfer tokens")]
    pub fn test_get_value_error() {
        Syscall::with_value_available(10);

        let state = RefCell::new(Default::default());
        let mut contract_service = ContractService::new(&state).expose(&[]);

        contract_service.get_value(15);
    }
}