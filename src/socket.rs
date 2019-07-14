use crate::live_view::LiveView;
use crate::live_view::Template;
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub kind: String,
    pub event: String,
    pub data: Option<String>,
}

/// Define http actor
pub struct StateSocket<State: Template> {
    pub state: State,
    pub live_view: LiveView<State>,
}

impl<State: Template> Actor for StateSocket<State> {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl<State: Template> StreamHandler<ws::Message, ws::ProtocolError> for StateSocket<State> {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => {
                let parsed: Event = serde_json::from_str(&text).unwrap();
                match parsed.kind.as_ref() {
                    "click" => click_handler(self, ctx, parsed),
                    "input" => input_handler(self, ctx, parsed),
                    "submit" => submit_handler(self, ctx, parsed),
                    _ => {}
                }
            }
            ws::Message::Binary(bin) => dbg!(ctx.binary(bin)),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}

fn click_handler<State: Template>(
    socket: &mut StateSocket<State>,
    ctx: &mut WebsocketContext<StateSocket<State>>,
    event: Event,
) {
    if let Some(f) = socket.live_view.click.get_mut(&event.event) {
        if let Some(rendered) = f(&event, &mut socket.state) {
            ctx.text(rendered);
        }
    }
}

fn submit_handler<State: Template>(
    socket: &mut StateSocket<State>,
    ctx: &mut WebsocketContext<StateSocket<State>>,
    event: Event,
) {
    if let Some(f) = socket.live_view.submit.get_mut(&event.event) {
        if let Some(rendered) = f(&event, &mut socket.state) {
            ctx.text(rendered);
        }
    }
}

fn input_handler<State: Template>(
    socket: &mut StateSocket<State>,
    ctx: &mut WebsocketContext<StateSocket<State>>,
    event: Event,
) {
    if let Some(f) = socket.live_view.input.get_mut(&event.event) {
        if let Some(rendered) = f(&event, &mut socket.state) {
            ctx.text(rendered);
        }
    }
}
