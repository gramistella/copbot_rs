#![allow(dead_code, unused_variables, unused_imports)]
extern crate gtk;
extern crate stopwatch;

use crate::req_handler;
use gtk::prelude::*;
use std::cell::Cell;
use stopwatch::Stopwatch;
use crate::bot;
use crate::log::LogHandler;

pub fn init_gtk(debug: bool) {

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("../resources/copbot.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    // main_window
    let main_window: gtk::Window = builder.get_object("main_window").unwrap();
    let start_button: gtk::Button = builder.get_object("start_button").unwrap();
    let entry_item: gtk::Entry = builder.get_object("text_item").unwrap();
    let entry_item_color: gtk::Entry = builder.get_object("text_item_color").unwrap();
    let size_combo: gtk::ComboBoxText = builder.get_object("size_combo").unwrap();

    // Menu
    let profiles_menu_item: gtk::MenuItem = builder.get_object("profiles_menu_item").unwrap();

    // Log
    let log_window: gtk::Window = builder.get_object("log_window").unwrap();
    let log_txt_buffer: gtk::TextView = builder.get_object("log_text_view").unwrap();
    let logger = LogHandler{txt_buffer: log_txt_buffer.get_buffer().unwrap(), debug};

    // profile_window
    let profile_window: gtk::Window = builder.get_object("profile_window").unwrap();
    let entry_profile: gtk::Entry = builder.get_object("text_profile").unwrap();
    let new_profile_combo: gtk::ComboBoxText = builder.get_object("new_profile_combo").unwrap();
    let profile_add_button: gtk::Button = builder.get_object("profile_add_button").unwrap();

    //let is_clicked = Cell::new(false);
    start_button.connect_clicked(move |_| {
        let sel_item = entry_item.get_text().to_string();
        let sel_item_color = entry_item_color.get_text().to_string();
        bot::start_bot(logger.clone(), sel_item, sel_item_color,size_combo.get_active_text().unwrap().to_string());
    });

    profiles_menu_item.connect_activate(move |_| {
        profile_window.show();
    });

    profile_add_button.connect_clicked(move |_| {

    });

    main_window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });


    log_window.show();
    main_window.show();

    if debug {
        gtk::Window::set_interactive_debugging(true);
    }

    gtk::main();
}