
// Add your service

use sails_rs::{cell::RefCell, prelude::*};
use sails_rs::collections::HashMap;
use sails_rs::gstd::msg;

#[derive(Clone, Default)]
pub struct State {
    pub greetings: HashMap<ActorId, String>,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoState {
    pub greetings: Vec<(ActorId, String)>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Events {
    Greeted { user: ActorId, message: String },
}

impl From<State> for IoState {
    fn from(value: State) -> Self {
        Self {
            greetings: value.greetings.into_iter().collect(),
        }
    }
}

pub struct Service<'a> {
    state: &'a RefCell<State>,
}

impl<'a> Service<'a> {
    pub fn new(state: &'a RefCell<State>) -> Self {
        Self { state }
    }

    fn get(&self) -> core::cell::Ref<'_, State> {
        self.state.borrow()
    }

    fn get_mut(&self) -> core::cell::RefMut<'_, State> {
        self.state.borrow_mut()
    }
}

#[service]
impl<'a> Service<'a> {
    #[export(route = "greet")]
    pub fn greet(&mut self) -> Events {
        let user = msg::source();
        let message = "Hello, World!".to_string();

        self.get_mut()
            .greetings
            .insert(user, message.clone());

        Events::Greeted { user, message }
    }

    #[export(route = "query")]
    pub fn query(&self) -> IoState {
        self.get().clone().into()
    }
}
