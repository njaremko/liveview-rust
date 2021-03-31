use crate::socket::Event;
use crate::Result;
use hashbrown::HashMap;

pub trait Template: Sized + 'static + Clone + Unpin {
    fn render(&self) -> Result<String>;
}

pub type EventHandler<State> = fn(&Event, &mut State) -> Option<String>;

#[derive(Default)]
pub struct LiveView<State: Template> {
    pub(crate) click: HashMap<String, EventHandler<State>>,
    pub(crate) submit: HashMap<String, EventHandler<State>>,
    pub(crate) input: HashMap<String, EventHandler<State>>,
    pub(crate) keydown: HashMap<String, EventHandler<State>>,
    pub(crate) mouseover: HashMap<String, EventHandler<State>>,
    pub(crate) mouseout: HashMap<String, EventHandler<State>>,
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

    pub fn on_keydown(&mut self, event: &str, func: EventHandler<State>) {
        self.keydown.insert(event.into(), func);
    }

    pub fn on_mouseover(&mut self, event: &str, func: EventHandler<State>) {
        self.mouseover.insert(event.into(), func);
    }

    pub fn on_mouseout(&mut self, event: &str, func: EventHandler<State>) {
        self.mouseout.insert(event.into(), func);
    }
}
