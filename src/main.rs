#![allow(non_snake_case)]
#![windows_subsystem = "windows"]
use crate::gui::init_gtk;
use fantoccini;
use std::process::Command;

mod gui;
mod req_handler;
mod bot_utils;
mod bot;
mod benchmark;
mod profileHandler;

#[tokio::main]
async fn main(){

    let debug = false;
    init_gtk(debug);

}
