use ::*;

pub enum CursorType {
    Default,
    Pointer,
    Drag,
}

impl Window {
    pub fn set_cursor_type(&self, cursor_type: CursorType) {
        #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        {
            let cursor_type = match cursor_type {
                CursorType::Default => "initial",
                CursorType::Pointer => "pointer",
                CursorType::Drag => "all-scroll",
            };
            // TODO: only canvas
            js! {
                document.body.style.cursor = @{cursor_type};
            };
        }
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        {
            use glutin::MouseCursor as GC;
            self.glutin_window.set_cursor(match cursor_type {
                CursorType::Default => GC::Default,
                CursorType::Pointer => GC::Hand,
                CursorType::Drag => GC::AllScroll,
            });
        };
    }

    pub fn set_cursor_position(&self, position: Vec2<f64>) {
        #![allow(unused_variables)]
        #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        unimplemented!();
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        self.glutin_window
            .set_cursor_position(position.x as i32, position.y as i32)
            .expect("Failed to set cursor position");
    }

    pub fn get_cursor_position(&self) -> Vec2<f64> {
        self.mouse_pos.get()
    }

    pub fn grab_cursor(&self) {
        #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        unimplemented!();
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        self.glutin_window
            .set_cursor_state(glutin::CursorState::Grab)
            .expect("Failed to grab cursor");
    }
}
