#![allow(dead_code, unused_variables, unused_imports)]
extern crate gtk;
extern crate stopwatch;
extern crate glib;
extern crate chrono;
extern crate gdk_pixbuf;

use glib::Bytes;
use chrono::prelude::*;
use crate::req_handler;
use gtk::prelude::*;
use std::cell::Cell;
use std::thread;
use stopwatch::Stopwatch;
use crate::bot;
use crate::profileHandler;
use std::borrow::Borrow;
use tokio::time::{delay_until, Instant};
use std::sync::atomic::{AtomicBool, Ordering, AtomicU64};
use std::sync::{Arc, Mutex};
use std::fs;
use fantoccini::Client;

pub fn init_gtk(debug: bool) {

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("../resources/copbot.glade");
    let builder = gtk::Builder::from_string(glade_src);

    // main_window
    let main_window: gtk::Window = builder.get_object("main_window").unwrap();
    let start_button: gtk::Button = builder.get_object("start_button").unwrap();
    let entry_item: gtk::Entry = builder.get_object("text_item").unwrap();
    let entry_item_color: gtk::Entry = builder.get_object("text_item_color").unwrap();
    let size_combo: gtk::ComboBoxText = builder.get_object("size_combo").unwrap();
    let profile_combo: gtk::ComboBoxText = builder.get_object("profile_combo").unwrap();
    let cc_combo: gtk::ComboBoxText = builder.get_object("cc_combo").unwrap();
    let gmail_combo: gtk::ComboBoxText = builder.get_object("gmail_combo").unwrap();
    let wait_checkbox: gtk::CheckButton = builder.get_object("timer_checkbutton").unwrap();

    populate_profile_combo(profile_combo.clone(), "None");
    populate_credit_combo(cc_combo.clone(), "None");
    populate_gmail_combo(gmail_combo.clone(), "None");

    // Menu
    let profiles_menu_item: gtk::MenuItem = builder.get_object("profiles_menu_item").unwrap();
    let credit_menu_item: gtk::MenuItem = builder.get_object("credit_menu_item").unwrap();
    let gmail_menu_item: gtk::MenuItem = builder.get_object("gmail_menu_item").unwrap();

    // Log
    let log_window: gtk::Window = builder.get_object("log_window").unwrap();
    let log_txt_view: gtk::TextView = builder.get_object("log_text_view").unwrap();
    let log_buffer = log_txt_view.get_buffer().unwrap();
    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    rx.attach(None, move |line: String| {
        let mut end_iter = log_buffer.get_end_iter();
        if line[..3] == " * ".to_string(){
            let mut prev_line = log_buffer.get_end_iter();
            prev_line.backward_line();
            log_buffer.delete(&mut prev_line, &mut end_iter);
            let mut end_iter = log_buffer.get_end_iter();
            let to_append = format!("{}\n", line);
            gtk::TextBufferExt::insert(&log_buffer, &mut end_iter, &to_append);
        } else {
            let mut end_iter = log_buffer.get_end_iter();
            let to_append = format!(" > {}\n", line);
            gtk::TextBufferExt::insert(&log_buffer, &mut end_iter, &to_append);
        }
        glib::Continue(true)
    });

    let atomic_stop = Arc::new(AtomicBool::new(false));
    let (send_thread, receive_thread) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let start_button_clone = start_button.clone();
    let atomic_stop_clone = atomic_stop.clone();
    receive_thread.attach(None, move |_: String| {
        gtk::ButtonExt::set_label(&start_button_clone, "Start");
        atomic_stop_clone.store(false, Ordering::SeqCst);
        glib::Continue(true)
    });

    // profile_window
    let profile_window: gtk::Window = builder.get_object("shipping_window").unwrap();
    let entry_profile: gtk::Entry = builder.get_object("text_profile").unwrap();
    let entry_full_name: gtk::Entry = builder.get_object("entry_full_name").unwrap();
    let entry_email: gtk::Entry = builder.get_object("entry_email").unwrap();
    let entry_tel: gtk::Entry = builder.get_object("entry_tel").unwrap();
    let entry_address: gtk::Entry = builder.get_object("entry_address").unwrap();
    let entry_city: gtk::Entry = builder.get_object("entry_city").unwrap();
    let entry_postcode: gtk::Entry = builder.get_object("entry_postcode").unwrap();
    let entry_country: gtk::Entry = builder.get_object("entry_country").unwrap();
    let new_profile_combo: gtk::ComboBoxText = builder.get_object("new_profile_combo").unwrap();
    let profile_add_button: gtk::Button = builder.get_object("profile_add_button").unwrap();
    let profile_remove_button: gtk::Button = builder.get_object("profile_remove_button").unwrap();

    // credit_window
    let cc_types = vec!["Visa", "American Express", "Mastercard", "Solo"];
    let exp_months = vec!["01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12"];
    let exp_years = vec!["2020", "2021", "2022", "2023", "2024", "2025", "2026", "2027", "2028", "2029", "2030"];
    let credit_window: gtk::Window = builder.get_object("card_window").unwrap();
    let new_credit_combo: gtk::ComboBoxText = builder.get_object("new_credit_combo").unwrap();
    let credit_profile_entry: gtk::Entry = builder.get_object("credit_profile_entry").unwrap();
    let credit_type: gtk::ComboBoxText = builder.get_object("credit_type_combo").unwrap();
    let credit_number: gtk::Entry = builder.get_object("credit_number_entry").unwrap();
    let credit_cvv: gtk::Entry = builder.get_object("credit_cvv_entry").unwrap();
    let credit_month: gtk::ComboBoxText = builder.get_object("credit_month_combo").unwrap();
    let credit_year: gtk::ComboBoxText = builder.get_object("credit_year_combo").unwrap();
    let credit_add_button: gtk::Button = builder.get_object("credit_add_button").unwrap();
    let credit_remove_button: gtk::Button = builder.get_object("credit_remove_button").unwrap();

    // gmail_window
    let gmail_window: gtk::Window = builder.get_object("gmail_window").unwrap();
    let new_gmail_combo: gtk::ComboBoxText = builder.get_object("new_gmail_combo").unwrap();
    let gmail_mail_entry: gtk::Entry = builder.get_object("entry_gmail_email").unwrap();
    let gmail_psw_entry: gtk::Entry = builder.get_object("entry_gmail_password").unwrap();
    let gmail_add_button: gtk::Button = builder.get_object("gmail_add_button").unwrap();
    let gmail_remove_button: gtk::Button = builder.get_object("gmail_remove_button").unwrap();

    let start_button_clone = start_button.clone();
    let profile_combo_clone = profile_combo.clone();
    let cc_combo_clone = cc_combo.clone();
    let size_combo_clone = size_combo.clone();
    let gmail_combo_clone = gmail_combo.clone();

    start_button.connect_clicked(move |_| {
        if gtk::ButtonExt::get_label(&start_button_clone).unwrap() == "Start" {
            let sel_item = entry_item.get_text().to_string();
            let sel_item_color = entry_item_color.get_text().to_string();
            let wait_for_drop = Arc::new(gtk::ToggleButtonExt::get_active(&wait_checkbox));
            let sel_size = size_combo_clone.get_active_text().unwrap().to_string();
            let tx_clone = tx.clone();
            let send_thread_clone = send_thread.clone();
            let sel_profile = profile_combo_clone.get_active_text().unwrap().to_string();
            let sel_credit_card = cc_combo_clone.get_active_text().unwrap().to_string();
            let sel_gmail_account = gmail_combo_clone.get_active_text().unwrap().to_string();
            let atomic_stop_clone = atomic_stop.clone();
            gtk::ButtonExt::set_label(&start_button_clone, "Stop");
            let bot_instance = thread::spawn(|| {
                bot::start_bot(tx_clone, send_thread_clone,atomic_stop_clone,sel_item, sel_item_color, sel_size, wait_for_drop, sel_profile, sel_credit_card, sel_gmail_account);
            });
        } else {
            atomic_stop.store(true, Ordering::SeqCst);
            gtk::ButtonExt::set_label(&start_button_clone, "Start");
        }
    });

    let new_profile_combo_clone = new_profile_combo.clone();
    let profile_window_clone = profile_window.clone();
    profiles_menu_item.connect_activate(move |_| {
        profile_window_clone.show();
        populate_profile_combo(new_profile_combo_clone.to_owned(), "New");
    });

    let new_credit_combo_clone = new_credit_combo.clone();
    let credit_window_clone = credit_window.clone();
    credit_menu_item.connect_activate(move |_| {
        credit_window_clone.show();
        populate_credit_combo(new_credit_combo_clone.to_owned(), "New");
    });

    let new_gmail_combo_clone = new_gmail_combo.clone();
    let gmail_window_clone = gmail_window.clone();
    gmail_menu_item.connect_activate(move |_| {
        gmail_window_clone.show();
        populate_gmail_combo(new_gmail_combo_clone.to_owned(), "New");
    });

    let new_profile_combo_clone = new_profile_combo.clone();
    let entry_profile_clone = entry_profile.clone();
    let entry_full_name_clone = entry_full_name.clone();
    let entry_email_clone = entry_email.clone();
    let entry_tel_clone = entry_tel.clone();
    let entry_address_clone = entry_address.clone();
    let entry_city_clone = entry_city.clone();
    let entry_postcode_clone = entry_postcode.clone();
    let entry_country_clone = entry_country.clone();
    let profile_add_button_clone = profile_add_button.clone();
    let profile_remove_button_clone = profile_remove_button.clone();

    new_profile_combo.connect_changed(move |_| {
        let mut profile_found = profileHandler::Profile::default();
        let sel_profile = match new_profile_combo_clone.get_active_text() {
            Some(option_string) => option_string.to_string(),
            None => return
        };
        profileHandler::get_profile(sel_profile.clone(), &mut profile_found).ok();
        if sel_profile != "New" {
            entry_profile_clone.set_text(&profile_found.profile_name);
            entry_full_name_clone.set_text(&profile_found.full_name);
            entry_email_clone.set_text(&profile_found.email);
            entry_tel_clone.set_text(&profile_found.tel);
            entry_address_clone.set_text(&profile_found.address);
            entry_city_clone.set_text(&profile_found.city);
            entry_postcode_clone.set_text(&profile_found.postcode);
            entry_country_clone.set_text(&profile_found.country);
            gtk::ButtonExt::set_label(&profile_add_button_clone, "Save");
            profile_remove_button_clone.show();
        } else {
            reset_profile_window(entry_profile_clone.clone(),
                                 entry_full_name_clone.clone(),
                                 entry_email_clone.clone(),
                                 entry_tel_clone.clone(),
                                 entry_address_clone.clone(),
                                 entry_city_clone.clone(),
                                 entry_postcode_clone.clone(),
                                 entry_country_clone.clone());

            gtk::ButtonExt::set_label(&profile_add_button_clone, "Add");
            profile_remove_button_clone.hide();
        }
    });

    let new_profile_combo_clone = new_profile_combo.clone();
    let entry_profile_clone = entry_profile.clone();
    let entry_full_name_clone = entry_full_name.clone();
    let entry_email_clone = entry_email.clone();
    let entry_tel_clone = entry_tel.clone();
    let entry_address_clone = entry_address.clone();
    let entry_city_clone = entry_city.clone();
    let entry_postcode_clone = entry_postcode.clone();
    let entry_country_clone = entry_country.clone();

    profile_add_button.connect_clicked(move |_| {
        let profile = profileHandler::Profile {
            profile_name: entry_profile_clone.get_text().to_string(),
            full_name: entry_full_name_clone.get_text().to_string(),
            email: entry_email_clone.get_text().to_string(),
            tel: entry_tel_clone.get_text().to_string(),
            address: entry_address_clone.get_text().to_string(),
            city: entry_city_clone.get_text().to_string(),
            postcode: entry_postcode_clone.get_text().to_string(),
            country: entry_country_clone.get_text().to_string()
        };
        reset_profile_window(entry_profile_clone.clone(),
                             entry_full_name_clone.clone(),
                             entry_email_clone.clone(),
                             entry_tel_clone.clone(),
                             entry_address_clone.clone(),
                             entry_city_clone.clone(),
                             entry_postcode_clone.clone(),
                             entry_country_clone.clone());
        profileHandler::store_profile(profile).ok();
        populate_profile_combo(new_profile_combo_clone.to_owned(), "New");
    });

    let new_profile_combo_clone = new_profile_combo.clone();
    let entry_profile_clone = entry_profile.clone();
    let entry_full_name_clone = entry_full_name.clone();
    let entry_email_clone = entry_email.clone();
    let entry_tel_clone = entry_tel.clone();
    let entry_address_clone = entry_address.clone();
    let entry_city_clone = entry_city.clone();
    let entry_postcode_clone = entry_postcode.clone();
    let entry_country_clone = entry_country.clone();

    profile_remove_button.connect_clicked(move |_| {
        let sel_profile = new_profile_combo_clone.get_active_text().unwrap().to_string();
        profileHandler::remove_profile(sel_profile).ok();
        reset_profile_window(entry_profile_clone.clone(),
                             entry_full_name_clone.clone(),
                             entry_email_clone.clone(),
                             entry_tel_clone.clone(),
                             entry_address_clone.clone(),
                             entry_city_clone.clone(),
                             entry_postcode_clone.clone(),
                             entry_country_clone.clone());

        populate_profile_combo(new_profile_combo_clone.to_owned(), "New");
    });

    let credit_profile_entry_clone = credit_profile_entry.clone();
    let credit_type_clone = credit_type.clone();
    let credit_number_clone = credit_number.clone();
    let credit_cvv_clone = credit_cvv.clone();
    let credit_month_clone = credit_month.clone();
    let credit_year_clone = credit_year.clone();
    let credit_add_button_clone = credit_add_button.clone();
    let credit_remove_button_clone = credit_remove_button.clone();
    let new_credit_combo_clone = new_credit_combo.clone();

    new_credit_combo.connect_changed(move |_| {
        let mut credit_card_found = profileHandler::CreditCard::default();
        let sel_credit_card_name = match new_credit_combo_clone.get_active_text() {
            Some(option_string) => option_string.to_string(),
            None => return
        };
        profileHandler::get_credit_card(sel_credit_card_name.clone(), &mut credit_card_found).ok();
        if sel_credit_card_name != "New" {

            let type_combo_idx = match cc_types.iter().position(|&r| r == credit_card_found._type){
                None => 0,
                Some(idx) => idx as u32
            };
            let month_combo_idx = match exp_months.iter().position(|&r| r == credit_card_found.month){
                None => 0,
                Some(idx) => idx as u32
            };
            let year_combo_idx = match exp_years.iter().position(|&r| r == credit_card_found.year){
                None => 0,
                Some(idx) => idx as u32
            };

            credit_profile_entry_clone.set_text(&credit_card_found.profile_name);
            credit_type_clone.set_active(Option::Some(type_combo_idx));
            credit_number_clone.set_text(&credit_card_found.number);
            credit_cvv_clone.set_text(&credit_card_found.cvv);
            credit_month_clone.set_active(Option::Some(month_combo_idx));
            credit_year_clone.set_active(Option::Some(year_combo_idx));

            gtk::ButtonExt::set_label(&credit_add_button_clone, "Save");
            credit_remove_button_clone.show();
        } else {

            credit_profile_entry_clone.set_text("");
            credit_type_clone.set_active(Option::Some(0));
            credit_number_clone.set_text("");
            credit_month_clone.set_active(Option::Some(0));
            credit_year_clone.set_active(Option::Some(0));
            credit_cvv_clone.set_text("");

            gtk::ButtonExt::set_label(&credit_add_button_clone, "Add");
            credit_remove_button_clone.hide();
        }
    });

    let credit_profile_entry_clone = credit_profile_entry.clone();
    let credit_type_clone = credit_type.clone();
    let credit_number_clone = credit_number.clone();
    let credit_cvv_clone = credit_cvv.clone();
    let credit_month_clone = credit_month.clone();
    let credit_year_clone = credit_year.clone();
    let new_credit_combo_clone = new_credit_combo.clone();

    credit_add_button.connect_clicked(move |_| {
        let credit = profileHandler::CreditCard {
            profile_name: credit_profile_entry_clone.get_text().to_string(),
            _type: credit_type_clone.get_active_text().unwrap().to_string(),
            number: credit_number_clone.get_text().to_string(),
            month: credit_month_clone.get_active_text().unwrap().to_string(),
            year: credit_year_clone.get_active_text().unwrap().to_string(),
            cvv: credit_cvv_clone.get_text().to_string(),
        };

        credit_profile_entry_clone.set_text("");
        credit_type_clone.set_active(Option::Some(0));
        credit_number_clone.set_text("");
        credit_month_clone.set_active(Option::Some(0));
        credit_year_clone.set_active(Option::Some(0));
        credit_cvv_clone.set_text("");

        profileHandler::store_credit(credit).ok();
        populate_credit_combo(new_credit_combo_clone.to_owned(), "None");
    });

    let new_gmail_combo_clone = new_gmail_combo.clone();
    let gmail_mail_entry_clone = gmail_mail_entry.clone();
    let gmail_psw_entry_clone = gmail_psw_entry.clone();

    gmail_add_button.connect_clicked(move |_| {
        let email =  gmail_mail_entry_clone.get_text().to_string();
        let email_split: Vec<&str> = email.split("@").collect();
        let user_account = profileHandler::Gmail {
            profile_name: email_split[0].to_string(),
            email,
            password: gmail_psw_entry_clone.get_text().to_string(),
        };

        gmail_mail_entry_clone.set_text("");
        gmail_psw_entry_clone.set_text("");

        profileHandler::store_gmail(user_account).ok();
        populate_gmail_combo(new_gmail_combo_clone.to_owned(), "None");
    });

    let new_gmail_combo_clone = new_gmail_combo.clone();
    let gmail_mail_entry_clone = gmail_mail_entry.clone();
    let gmail_psw_entry_clone = gmail_psw_entry.clone();

    gmail_remove_button.connect_clicked(move |_| {
        let sel_account = new_gmail_combo_clone.get_active_text().unwrap().to_string();
        profileHandler::remove_gmail(sel_account).ok();
        gmail_mail_entry_clone.set_text("");
        gmail_psw_entry_clone.set_text("");
        populate_gmail_combo(new_gmail_combo_clone.to_owned(), "New");
    });

    let new_gmail_combo_clone = new_gmail_combo.clone();
    let gmail_mail_entry_clone = gmail_mail_entry.clone();
    let gmail_psw_entry_clone = gmail_psw_entry.clone();
    let gmail_add_button_clone = gmail_add_button.clone();
    let gmail_remove_button_clone = gmail_remove_button.clone();
    new_gmail_combo.connect_changed(move |_| {
        let mut profile_found = profileHandler::Gmail::default();
        let sel_profile = match new_gmail_combo_clone.get_active_text() {
            Some(option_string) => option_string.to_string(),
            None => return
        };
        profileHandler::get_gmail_account(sel_profile.clone(), &mut profile_found).ok();
        if sel_profile != "New" {
            gmail_mail_entry_clone.set_text(&profile_found.email);
            gmail_psw_entry_clone.set_text(&profile_found.password);

            gtk::ButtonExt::set_label(&gmail_add_button_clone, "Save");
            gmail_remove_button_clone.show();
        } else {
            gmail_mail_entry_clone.set_text("");
            gmail_psw_entry_clone.set_text("");
            gtk::ButtonExt::set_label(&gmail_add_button_clone, "Add");
            gmail_remove_button_clone.hide();
        }
    });

    let credit_profile_entry_clone = credit_profile_entry.clone();
    let credit_type_clone = credit_type.clone();
    let credit_number_clone = credit_number.clone();
    let credit_cvv_clone = credit_cvv.clone();
    let credit_month_clone = credit_month.clone();
    let credit_year_clone = credit_year.clone();
    let new_credit_combo_clone = new_credit_combo.clone();
    credit_remove_button.connect_clicked(move |_| {
        let sel_credit = new_credit_combo_clone.get_active_text().unwrap().to_string();
        profileHandler::remove_credit(sel_credit).ok();
        credit_profile_entry_clone.set_text("");
        credit_type_clone.set_active(Option::Some(0));
        credit_number_clone.set_text("");
        credit_month_clone.set_active(Option::Some(0));
        credit_year_clone.set_active(Option::Some(0));
        credit_cvv_clone.set_text("");

        populate_credit_combo(new_credit_combo_clone.to_owned(), "New");
    });

    main_window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let profile_window_clone = profile_window.clone();
    profile_window.connect_delete_event(move |_, _| {
        profile_window_clone.hide();
        populate_profile_combo(profile_combo.clone(), "None");
        Inhibit(true)
    });

    let credit_window_clone = credit_window.clone();
    credit_window.connect_delete_event(move |_, _| {
        credit_window_clone.hide();
        populate_credit_combo(cc_combo.clone(), "None");
        Inhibit(true)
    });

    let gmail_window_clone = gmail_window.clone();
    gmail_window.connect_delete_event(move |_, _| {
        gmail_window_clone.hide();
        populate_gmail_combo(gmail_combo.clone(), "None");
        Inhibit(true)
    });

    log_window.show();
    main_window.show();

    if debug {
        gtk::Window::set_interactive_debugging(true);
    }

    gtk::main();
}

fn reset_profile_window(entry_profile_clone: gtk::Entry, entry_full_name_clone: gtk::Entry, entry_email_clone: gtk::Entry, entry_tel_clone: gtk::Entry, entry_address_clone: gtk::Entry, entry_city_clone: gtk::Entry, entry_postcode_clone: gtk::Entry, entry_country_clone: gtk::Entry) {
    entry_profile_clone.set_text("");
    entry_full_name_clone.set_text("");
    entry_email_clone.set_text("");
    entry_tel_clone.set_text("");
    entry_address_clone.set_text("");
    entry_city_clone.set_text("");
    entry_postcode_clone.set_text("");
    entry_country_clone.set_text("");
}

fn populate_profile_combo(combo: gtk::ComboBoxText, init_string: &str) -> bool{
    combo.remove_all();
    combo.append_text(&init_string);
    combo.set_active(Option::Some(0));
    let mut profiles: Vec<profileHandler::Profile> = Vec::new();
    profileHandler::get_profiles(&mut profiles).ok();
    for profile in &profiles{
        combo.append_text(profile.profile_name.as_ref());
    }
    if profiles.len() == 0 {
        return false;
    }
    return true;
}

fn populate_credit_combo(combo: gtk::ComboBoxText, init_string: &str) -> bool{
    combo.remove_all();
    combo.append_text(init_string);
    combo.set_active(Option::Some(0));
    let mut profiles: Vec<profileHandler::CreditCard> = Vec::new();
    profileHandler::get_credit_cards(&mut profiles).ok();
    for profile in &profiles{
        combo.append_text(profile.profile_name.as_ref());
    }
    if profiles.len() == 0 {
        return false;
    }
    return true;
}

fn populate_gmail_combo(combo: gtk::ComboBoxText, init_string: &str) -> bool{
    combo.remove_all();
    combo.append_text(init_string);
    combo.set_active(Option::Some(0));
    let mut profiles: Vec<profileHandler::Gmail> = Vec::new();
    profileHandler::get_gmail_accounts(&mut profiles).ok();
    for profile in &profiles{
        combo.append_text(&format!("{}..", &profile.profile_name[..10]));
    }
    if profiles.len() == 0 {
        return false;
    }
    return true;
}