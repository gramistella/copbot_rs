extern crate reqwest;

use crate::req_handler;
use crate::bot_utils;
use crate::bot_utils::{ShopItem, BotItem, SelectedBotItem};
use crate::log::LogHandler;
use reqwest::ClientBuilder;
use reqwest::header;
use stopwatch::Stopwatch;



pub fn start_bot(logger: LogHandler, item_kw: String, item_color: String, sel_size: String){

    let mut headers = header::HeaderMap::new();
    bot_utils::gen_header_map(&mut headers);

    let mut client = ClientBuilder::new()
        .default_headers(headers)
        .cookie_store(true)
        .build().unwrap();

    logger.print_to_log(&format!("Selected item: {} , color {}", item_kw,  if item_color.clone() != "" {item_color.clone()} else { "Any".to_string() }));

    let mut shop_item_array: Vec<ShopItem> = Vec::new();
    // Populate array
    let mut sw = Stopwatch::start_new();
    req_handler::fetch_shop_items(&mut shop_item_array).ok();

    // Find the item
    let item = find_requested_item(&mut client, &mut shop_item_array, item_kw, item_color, sel_size);

    if item.status > 0 {
        if item.status == 1 {
            logger.print_to_log("Unable to find item :/");
        } else if item.status == 2{
            logger.print_to_log(&format!("Found {} ({})", item.name, item.color));
            logger.print_to_log("Item is sold-out :(");
        } else {}
    } else {
        logger.print_to_log(&format!("Attempting to purchase: {} ({}) {} | id: {} style_id: {} , size_id: {} ", item.name, item.color, item.size, item.id, item.style_id, item.size_id));
        req_handler::add_to_cart(&mut client, logger.clone(), item).ok();
        logger.print_to_log(&format!("Took {}ms", sw.elapsed_ms() ));
    }
    sw.stop();

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