use gl;

use common::*;

pub enum Command {
    Clear { color: Color },
}

pub fn immediate(command: Command) {
    match command {
        Command::Clear {
            color: Color {
                red,
                green,
                blue,
                alpha,
            },
        } => unsafe {
            gl::ClearColor(red, green, blue, alpha);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        },
    }
}