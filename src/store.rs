use futures_signals::signal::Mutable;
use log::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use yew::agent::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    pub ip: Mutable<Option<String>>,
}

pub type ArcState = Arc<State>;

#[derive(Deserialize, Serialize)]
pub enum StoreInput {
    Action,
    Mutation,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StoreOutput {
    StateInstance(Arc<State>),
}

pub struct Store {
    link: AgentLink<Store>,
    state: Arc<State>,
}

impl Store {
    fn init_state() -> Arc<State> {
        Arc::new(State {
            ip: Mutable::new(None),
        })
    }
}

impl Agent for Store {
    type Reach = Context;
    type Message = ();
    type Input = StoreInput;
    type Output = StoreOutput;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            state: Self::init_state(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            StoreInput::Action => self.state.as_ref().ip.set(Some(String::from("1.1.1.1"))),
            _ => {}
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.link
            .respond(id, StoreOutput::StateInstance(self.state.clone()));
    }
}
