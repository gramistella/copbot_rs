#![allow(dead_code, unused_variables, unused_imports)]
extern crate reqwest;
extern crate tokio;
extern crate scraper;
extern crate serde_json;

use scraper::{Html, Selector};
use crate::bot_utils::{ShopItem, ItemSize, BotItem, SelectedBotItem};
use crate::log::LogHandler;
use std::borrow::{BorrowMut, Borrow};
use gtk::prelude::ToSendValue;

#[tokio::main]
pub async fn fetch_shop_items(item_array: &mut Vec<ShopItem>) -> Result<(), reqwest::Error> {
    let url = format!("https://www.supremenewyork.com/mobile_stock.json");
    let body = reqwest::get(&url)
        .await?
        .text()
        .await?;
    let json: serde_json::Value = serde_json::from_str(&body).expect("JSON was not well-formatted");
    for cat_item_list in json["products_and_categories"].as_object().unwrap().values(){
        for item_list in cat_item_list.as_array().iter(){
            for item in item_list.iter(){

                let item_struct = ShopItem {
                    name: item["name"].to_string(),
                    id: item["id"].as_i64().unwrap(),
                    matched_item_keywords: 0
                };
                item_array.push(item_struct);
                //println!("{} ({}) | {}", item["name"], item["color"], item["id"]);
            }
        }


    }
    Ok(())
}

#[tokio::main]
pub async fn fetch_item_details(client: &mut reqwest::Client, item: &mut ShopItem, user_color: String, sel_item_styles: &mut Vec<BotItem>) -> Result<(), reqwest::Error> {
    let url = format!("https://www.supremenewyork.com/shop/{}.json", item.id);
    let body = client.get(&url).send().await?.text().await?;
    let json: serde_json::Value = serde_json::from_str(&body).expect("JSON was not well-formatted");
    for styles in json["styles"].as_array().iter(){
        for single_style in styles.iter(){
            let style_color = single_style["name"].as_str().unwrap().trim();
            let style_id = single_style["id"].as_i64().unwrap();
            //println!("{} - {}", style_color, single_style["id"]);
            let mut sizes_vec: Vec<ItemSize> = Vec::new();
            for sizes in single_style["sizes"].as_array().iter(){
                for size in sizes.iter(){
                    let name = size["name"].as_str().unwrap().trim();
                    let id = size["id"].as_i64().unwrap();
                    let in_stock = size["stock_level"].as_i64().unwrap() != 0;
                    let size_struct = ItemSize{
                        name: name.to_string(),
                        id,
                        in_stock
                    };
                    sizes_vec.push(size_struct);
                    //println!("- Size {} : {} | in stock: {}", name, id, in_stock);
                };
            }
            let item_struct = BotItem{
                name: item.name.to_owned(),
                color: style_color.to_string(),
                matched_item_keywords: item.matched_item_keywords,
                matched_color_keywords: 0,
                id: item.id,
                style_id,
                sizes: sizes_vec
            };
            sel_item_styles.push(item_struct);
        }
    }

    Ok(())
}
#[tokio::main]
pub async fn add_to_cart(client: &mut reqwest::Client, logger: LogHandler, item: SelectedBotItem) -> Result<(), reqwest::Error>{
    // size=64905&style=29235&qty=1
    let url = format!("https://www.supremenewyork.com/shop/{}/add.json", item.id);
    let post_body = format!("size={}&style={}&qty=1", item.size_id, item.style_id);
    let response = client.post(&url).body(post_body).send().await?.text().await?;
    // Looks like if the item is sold out it returns []
    if response != "[]"{
        logger.print_to_log("Successfully added to cart");
    }
    Ok(())
}

// #[tokio::main]
// pub async fn check_cart(client: &mut reqwest::Client) -> Result<(), reqwest::Error>{
//
//     let url = "https://www.supremenewyork.com/checkout/totals_mobile.json";
//     let body = client.get(url).send().await?.text().await?;
//     if body != "{}"{
//         println!("Item added to cart successfully");
//     }
//
//     Ok(())
//
// }