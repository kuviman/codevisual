use *;

pub trait App: 'static {
    fn title() -> String {
        String::from("CodeVisual application")
    }
    fn new(context: &Rc<Context>) -> Self;
    #[allow(unused_variables)]
    fn update(&mut self, delta_time: f64) {}
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer);

    #[allow(unused_variables)]
    fn handle_event(&mut self, event: Event) {}
}
