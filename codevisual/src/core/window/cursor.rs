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
            run_js! {
                CodeVisual.internal.set_cursor(match cursor_type {
                    Default => "initial",
                    Pointer => "pointer",
                    Drag => "all-scroll",
                });
            }
        };
        #[cfg(not(target_os = "emscripten"))]
        {
            use glutin::MouseCursor as GC;
            self.glutin_window.set_cursor(match cursor_type {
                Default => GC::Default,
                Pointer => GC::Hand,
                Drag => GC::AllScroll,
            });
        };
    }

    pub fn set_cursor_position(&self, position: Vec2) {
        #![allow(unused_variables)]
        #[cfg(target_os = "emscripten")]
        unimplemented!();
        #[cfg(not(target_os = "emscripten"))]
        self.glutin_window.set_cursor_position(position.x as i32, position.y as i32)
            .expect("Failed to set cursor position");
    }

    pub fn get_cursor_position(&self) -> Vec2 {
        self.mouse_pos.get()
    }

    pub fn grab_cursor(&self) {
        #[cfg(target_os = "emscripten")]
        unimplemented!();
        #[cfg(not(target_os = "emscripten"))]
        self.glutin_window.set_cursor_state(glutin::CursorState::Grab)
            .expect("Failed to grab cursor");
    }
}
