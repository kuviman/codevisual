use ::*;

impl Window {
    pub(crate) fn subscribe_events<F: Fn(Event)>(&self, _handler: F) {
        let _ = self.canvas;
    }
}
