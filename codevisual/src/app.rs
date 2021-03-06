use *;

pub trait App: 'static {
    #[allow(unused_variables)]
    fn update(&mut self, delta_time: f64) {}
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer);

    #[allow(unused_variables)]
    fn handle_event(&mut self, event: Event) {}
}
