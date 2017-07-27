use ::*;

pub enum CursorType {
    Default,
    Pointer,
    Drag,
}

impl Window {
    pub fn set_cursor_type(&self, cursor_type: CursorType) {
        use CursorType::*;
        #[cfg(target_os = "emscripten")]
        {
            run_js!{
                CodeVisual.internal.set_cursor(match cursor_type {
                    Default => "initial",
                    Pointer => "pointer",
                    Drag => "all-scroll",
                });
            }
        }
        #[cfg(not(target_os = "emscripten"))]
        {
            use glutin::MouseCursor as GC;
            self.glutin_window
                .set_cursor(match cursor_type {
                                Default => GC::Default,
                                Pointer => GC::Hand,
                                Drag => GC::AllScroll,
                            });
        }
    }
}