#[macro_use]
extern crate codevisual;

mod src;
use src::*;

fn main() {
    codevisual::run::<Test>();
}