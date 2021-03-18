extern crate reqwest;
extern crate chrono;
extern crate spin_sleep;
extern crate glib;
extern crate failure;

use chrono::prelude::*;
use crate::req_handler;
use crate::bot_utils;
use crate::bot_utils::{ShopItem, BotItem, SelectedBotItem, legit_type_str};
use crate::profileHandler;
use reqwest::ClientBuilder;
use reqwest::header;
use stopwatch::Stopwatch;
use glib::Sender;
use cancellable_timer::*;
use tokio::time::Delay;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering, AtomicU64};
use self::chrono::Duration;
use std::thread::sleep;
use fantoccini::Locator;
use std::fs;
use headless_chrome::{Browser, Tab, LaunchOptionsBuilder};


pub fn start_bot(tx: Sender<String>, send_thread: Sender<String>, stop: Arc<AtomicBool>, item_kw: String, item_color: String, sel_size: String, wait: Arc<bool>, sel_profile_str: String, sel_credit_card_str: String, sel_gmail_str: String) -> Result<(), failure::Error>{

    let mut scraping_headers = header::HeaderMap::new();
    let mut checkout_headers = header::HeaderMap::new();
    bot_utils::gen_scraping_header_map(&mut scraping_headers);
    bot_utils::gen_checkout_header_map(&mut checkout_headers);

    let mut client = ClientBuilder::new()
        .default_headers(scraping_headers)
        .cookie_store(true)
        .build().unwrap();

    // let mut browser =  fantoccini::Client::new("http://localhost:4444").await.expect("Failed to connect to WebDriver");
    // browser.goto("https://patrickhlauke.github.io/recaptcha/").await?;
    // sleep(std::time::Duration::from_millis(1000));
    // let mut browser = browser.enter_frame(Some(0)).await.expect("couldn't find frame");
    // browser.find(Locator::Css("#recaptcha-anchor")).await?.click().await?;
    // loop {
    //     let screenshot = browser.screenshot().await?;
    //     fs::write("screenshot.jpg", &screenshot)?;
    //     println!("{:?} {:?}",atomic_x, atomic_y);
    //     browser.execute(&format!("document.elementFromPoint({:?}, {:?}).click();", atomic_y, atomic_x), Vec::new());
    //     let recaptcha_class = browser.find(Locator::Css("#recaptcha-anchor")).await?.attr("class").await?.unwrap();
    //     if recaptcha_class.contains("recaptcha-checkbox-checked") {
    //         break
    //     }
    // }
    let browser = Browser::new(LaunchOptionsBuilder::default().args(vec![std::ffi::OsStr::new("-disable-extensions")]).headless(false).build().unwrap()).ok().unwrap();
    let mut tab = browser.wait_for_initial_tab()?;
    let user_agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 13_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/80.0.3987.95 Mobile/15E148 Safari/604.1";
    tab.set_user_agent(user_agent, Option::from("en-US,en;q=0.5"), Option::from("iPhone; CPU iPhone OS 13_3 like Mac OS X"));

    let mut sel_profile = profileHandler::Profile::default();
    let mut sel_credit_card = profileHandler::CreditCard::default();
    let mut sel_gmail_account = profileHandler::Gmail::default();

    profileHandler::get_profile(sel_profile_str, &mut sel_profile).ok();
    profileHandler::get_credit_card(sel_credit_card_str, &mut sel_credit_card).ok();
    profileHandler::get_gmail_account(sel_gmail_str.clone(), &mut sel_gmail_account).ok();

    tx.send(format!("Selected item: {} , color {}", item_kw,  if item_color.clone() != "" {item_color.clone()} else { "Any".to_string() })).expect("Couldn't send data to channel");

    let mut shop_item_array: Vec<ShopItem> = Vec::new();

    if sel_gmail_str != "None" {
        tab.navigate_to("https://stackoverflow.com/users/login");
        sleep(std::time::Duration::from_millis(1500));
        while tab.get_url().contains("captcha") { sleep(std::time::Duration::from_millis(600)); }
        tab.wait_for_element("button[data-provider=\"google\"]")?.click()?;
        sleep(std::time::Duration::from_millis(3500));
        tab.wait_for_element("div > input[type=\"email\"]")?.click()?;
        legit_type_str(tab.clone(), &sel_gmail_account.email);
        tab.press_key("Enter")?;
        sleep(std::time::Duration::from_millis(2500));
        tab.wait_for_element("div > input[type=\"password\"]")?.click()?;
        legit_type_str(tab.clone(), &sel_gmail_account.password);
        tab.press_key("Enter")?;
        sleep(std::time::Duration::from_millis(2500));

    }
    if *wait {
        let mut timer_dur = std::time::Duration::from_secs(1);

        while timer_dur.as_millis() > 0 {

            if stop.load(Ordering::SeqCst) {
                break;
            }
            let drop_date = Local::now();
            let drop_time = (drop_date).date().and_hms(11, 59, 50);
            //let drop_time = (drop_date).date().and_hms(19, 44, 00);
            timer_dur = match drop_time.signed_duration_since(drop_date).to_std() {
                Err(_E) => { break },
                Ok(dur) => dur
            };

            let seconds = timer_dur.as_secs() % 60;
            let minutes = (timer_dur.as_secs() / 60) % 60;
            let hours = (timer_dur.as_secs() / 60) / 60;
            tx.send(format!(" * Waiting for drop ({:02}:{:02}:{:02})",
                            hours,
                            minutes,
                            seconds)
            ).expect("Couldn't send data to channel");
            sleep(std::time::Duration::from_millis(300));

        }

        if stop.load(Ordering::SeqCst){
            tx.send(format!("Stopping thread...")).expect("Couldn't send data to channel");
            send_thread.send("".to_string()).expect("Couldn't send data to thread handling channel");
            return Ok(())
        }
        tx.send(format!("Refreshing website...")).expect("Couldn't send data to channel");
        req_handler::wait_for_drop(tx.clone()).ok();
    }
    // Populate array
    let mut sw = Stopwatch::start_new();
    let ret = req_handler::fetch_shop_items(&mut shop_item_array).ok().unwrap();

    if ret == false {
        tx.send(format!("Couldn't fetch shop items (Webstore is closed)")).expect("Couldn't send data to channel");
    } else {

        // Find the item
        let item = find_requested_item(&mut client, &mut shop_item_array, item_kw, item_color, sel_size);

        if item.status > 0 {
            if item.status == 1 {
                tx.send("Unable to find item :/".to_string()).expect("Couldn't send data to channel");
            } else if item.status == 2 {
                tx.send(format!("Found {} ({})", item.name, item.color)).expect("Couldn't send data to channel");
                tx.send("Item is sold-out :(".to_string()).expect("Couldn't send data to channel");
            } else {}
        } else {
            tx.send(format!("Attempting to purchase: {} ({}) {} | id: {} style_id: {} , size_id: {} ", item.name, item.color, item.size, item.id, item.style_id, item.size_id)).expect("Couldn't send data to channel");
            req_handler::add_to_cart(&mut client, tab.clone(),tx.clone(), item.clone()).ok();

            tx.send(format!("Took {}ms", sw.elapsed_ms())).expect("Couldn't send data to channel");

            tx.send("Checking out...".to_string()).expect("Couldn't send data to channel");
            let status = req_handler::checkout(&mut client, tab, item, sel_profile, sel_credit_card).unwrap();
            tx.send(format!("Checked out with status: {}", status));
            //
            // tx.send("Getting order status...".to_string()).expect("Couldn't send data to channel");
            // let order_status = req_handler::get_order_status(&mut client, order_id).unwrap_or_else(|_| "failed".to_string());
            //
            // tx.send(format!("Order status: {}", order_status )).expect("Couldn't send data to channel");
        }
        sw.stop();
        send_thread.send("".to_string()).expect("Couldn't send data to thread handling channel");
    }


    sleep(std::time::Duration::from_millis(5000));
    Ok(())
}

pub fn find_requested_item(mut client: &mut reqwest::Client, item_array: &mut Vec<ShopItem>, item_kw: String, item_color: String, item_size: String) -> SelectedBotItem{

    // Determine most likely item
    for item in item_array.iter_mut(){
        bot_utils::compare_name_keywords(item, item_kw.clone());
    }

    let mut best_item_match = ShopItem::default();
    for item in item_array.iter(){
        bot_utils::find_most_likely_item(item.clone(), &mut best_item_match);
    }

    // Fetch item details
    let mut sel_item = SelectedBotItem::default();
    if best_item_match.matched_item_keywords > 0 {
        let mut sel_item_styles: Vec<BotItem> = Vec::new();
        req_handler::fetch_item_details(&mut client, &mut best_item_match, item_color.clone(), &mut sel_item_styles).ok();

        // Determine which color is requested or todo get any color
        for mut item in sel_item_styles.iter_mut() {
            bot_utils::compare_color_keywords(&mut item, item_color.clone());
        }
        let mut best_match = BotItem::default();
        for item in sel_item_styles.iter() {
            bot_utils::find_most_likely_color(item.clone(), &mut best_match);
        }

        if best_match.matched_color_keywords > 0 || item_color == "" {

            // Determine which sizes are available
            sel_item = bot_utils::find_available_size(&mut best_match, item_size);

        } else {
            sel_item.status = 1;
        }
    } else {
        sel_item.status = 1;
    }
    return sel_item;
}