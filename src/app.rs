use futures::future::ready;
use futures_signals::signal::SignalExt;
use log::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::prelude::*;

use super::store::{ActionType::GetIp, ArcState, Store, StoreInput, StoreOutput};
use super::subscriber::Subscriber;

pub struct App {
    ip: Option<String>,
    link: ComponentLink<App>,
    store: Box<dyn Bridge<Store>>,
    state_ref: Option<ArcState>,
    total_subs: i32,
}

pub enum Msg {
    FromStore(StoreOutput),
    GetIp,
    SetIp(Option<String>),
    IncSubs,
    DecSubs,
}

impl App {
    fn register_state_handlers(&self) {
        let callback = self.link.callback(|ip| Msg::SetIp(ip));
        let state = self.state_ref.as_ref().unwrap();
        let handler = state.ip.signal_cloned().for_each(move |u| {
            info!("{:?}", u);
            callback.emit(u);
            ready(())
        });
        spawn_local(handler);
    }

    fn get_ip(&self) -> Callback<MouseEvent> {
        self.link.callback(|_| Msg::GetIp)
    }

    fn add_sub(&self) -> Callback<MouseEvent> {
        self.link.callback(|_| Msg::IncSubs)
    }

    fn dec_sub(&self) -> Callback<MouseEvent> {
        self.link.callback(|_| Msg::DecSubs)
    }
}

impl Component for App {
    type Properties = ();
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let store = Store::bridge(link.callback(|d| Msg::FromStore(d)));

        Self {
            ip: None,
            link,
            store,
            state_ref: None,
            total_subs: 2,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.register_state_handlers();
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FromStore(s) => match s {
                StoreOutput::StateInstance(state) => {
                    self.state_ref = Some(state);
                    true
                }
            },
            Msg::GetIp => {
                self.store.send(StoreInput::Action(GetIp));
                false
            }
            Msg::SetIp(ip) => {
                self.ip = ip;
                true
            }
            Msg::IncSubs => {
                self.total_subs += 1;
                true
            }
            Msg::DecSubs => {
                if self.total_subs - 1 >= 1 {
                    self.total_subs -= 1;
                }
                true
            }
        }
    }

    fn view(&self) -> Html {
        let message = String::from("No ip, please click the button");
        let ip = if self.ip.is_some() {
            self.ip.as_ref().unwrap()
        } else {
            &message
        };
        let subs = (0..self.total_subs).map(|x| html! { <Subscriber id=x /> });
        html! {
            <div class="app-container">
                <h2>{{ "Click the button to get your ip" }}</h2>
                <p>{{ ip }}</p>
                <div class="buttons">
                    <button onclick=&self.get_ip()>{{ "Get ip" }}</button>
                    <button onclick=&self.add_sub()>{{ "Add subscriber" }}</button>
                    <button onclick=&self.dec_sub()>{{ "Remove Subscriber" }}</button>

                </div>

                { for subs }
            </div>
        }
    }
}
