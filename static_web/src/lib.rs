#![deny(warnings)]

pub const DTS: &str = include_str!(concat!(env!("OUT_DIR"), "/lib.d.ts"));
pub const HTML: &str = include_str!(concat!(env!("OUT_DIR"), "/lib.html"));
pub const CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/lib.css"));
pub const JS: &str = include_str!(concat!(env!("OUT_DIR"), "/lib.js"));