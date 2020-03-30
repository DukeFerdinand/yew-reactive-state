## About

You can use this as a minimal template for any of your web applications. The base for the state system here is a `yew` Agent and the `Mutable` wrapper from `futures_signals`.

The flow is as follows:

- App Component (or other higher up component) makes first connection with `Store` Agent.
- `Store` agent is created, instantiates `State` object with `Mutable` field(s).
- Store sends `StateInstance(State)` back to `App` (or other connected components on connect)
- Component can then subscribe to any updates it cares about

Here's an example on how to subscribe to updates on a `String` field:

```rust
impl App {
    fn register_state_handlers(&self) {
        let callback = self.link.callback(|ip| Msg::SetIp(ip));
        let state = self.state_ref.as_ref().unwrap();
        let handler = state.ip.signal_cloned().for_each(move |u| {
          info!("{:?}", u); // from log crate
            callback.emit(u);
            ready(()) // from futures crate
        });
        // for_each converts the signals into futures, so you'll need to spawn that
        spawn_local(handler); // from wasm_bindgen_futures
    }
}

// ... rest of your component implementation
```

The `State` object in this case would look something like this:

```rust
struct State {
  ip: Mutable<Option<String>>
}
```

## For More info on `futures_signals`

You can use the tutorial for the library [here](https://docs.rs/futures-signals/0.3.15/futures_signals/tutorial/index.html)
