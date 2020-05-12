#![allow(non_snake_case)]
#![windows_subsystem = "windows"]
use crate::gui::init_gtk;

mod gui;
mod req_handler;
mod bot_utils;
mod bot;
mod log;
mod benchmark;

fn main() {

    let debug = true;
    init_gtk(debug);
    //benchmark::start_benchmark(20);
}
