// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(lazy_cell)]

mod consts;

#[allow(inactive_code)]
#[cfg(feature = "gui")]
mod gui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "gui")]
    return Ok(gui::main()?);

    todo!()
}
