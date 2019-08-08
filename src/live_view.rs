use crate::socket::Event;
use crate::Result;
use hashbrown::HashMap;

pub trait Template: Sized + 'static + Clone {
    fn render(&self) -> Result<String>;
}

pub type EventHandler<State> = fn(&Event, &mut State) -> Option<String>;

#[derive(Default)]
pub struct LiveView<State: Template> {
    pub(crate) click: HashMap<String, EventHandler<State>>,
    pub(crate) submit: HashMap<String, EventHandler<State>>,
    pub(crate) input: HashMap<String, EventHandler<State>>,
}

impl<State: Template> LiveView<State> {
    pub fn on_click(&mut self, event: &str, func: EventHandler<State>) {
        self.click.insert(event.into(), func);
    }

    pub fn on_submit(&mut self, event: &str, func: EventHandler<State>) {
        self.submit.insert(event.into(), func);
    }

    pub fn on_input(&mut self, event: &str, func: EventHandler<State>) {
        self.input.insert(event.into(), func);
    }
}
