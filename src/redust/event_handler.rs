use crate::redust::event::Event;

pub struct EventHandler<'t> {
    event: &'t Event,
    callback: &'t (dyn Fn(String) + Send + Sync + 'static),
}

impl<'t> EventHandler<'t> {
    pub fn new(
        event: &'t Event,
        callback: &'t (dyn Fn(String) + Send + Sync + 'static),
    ) -> EventHandler<'t> {
        EventHandler { event, callback }
    }

    pub fn get_event(&self) -> &Event {
        self.event
    }

    pub fn get_callback(&self) -> &(dyn Fn(String) + Send + Sync + 'static) {
        self.callback
    }
}
