#![allow(dead_code, unused_variables, unused_imports)]
extern crate reqwest;
extern crate tokio;
extern crate scraper;
extern crate serde_json;
extern crate glib;
extern crate failure;
extern crate fantoccini;

use soup::prelude::*;
use scraper::{Html, Selector, ElementRef};
use crate::bot_utils::{ShopItem, ItemSize, BotItem, SelectedBotItem, crop_letters, legit_type_into};
use crate::profileHandler::{CreditCard, Profile, write_to_debug};
use std::borrow::{BorrowMut, Borrow};
use std::collections::hash_map::HashMap;
use gtk::prelude::ToSendValue;
use glib::Sender;
use chrono::Local;
use std::thread::sleep;
use url::form_urlencoded;
use self::reqwest::get;
use self::reqwest::header::SET_COOKIE;
use cookie::Cookie;
use std::sync::{Arc, Mutex};
use std::fs;
use headless_chrome::Tab;
use headless_chrome::protocol::dom::NodeAttributes;
use headless_chrome::protocol::network::{Request};
use headless_chrome::protocol::network::events::ResourceType::XHR;
use headless_chrome::protocol::network::methods::RequestPattern;
use headless_chrome::browser::tab::RequestInterceptor;
use stopwatch::Stopwatch;

#[tokio::main]
pub async fn fetch_shop_items(item_array: &mut Vec<ShopItem>) -> Result<bool, reqwest::Error> {
    let url = format!("https://www.supremenewyork.com/mobile_stock.json");
    let body = reqwest::get(&url)
        .await?
        .text()
        .await?;
    let json: serde_json::Value = serde_json::from_str(&body).expect("JSON was not well-formatted");
    let products_and_categories = json["products_and_categories"].as_object().unwrap().values();

    // Webstore is closed
    if products_and_categories.len() == 0 {
        return Ok(false)
    }
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
    Ok(true)
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
pub async fn add_to_cart(client: &mut reqwest::Client, tab: Arc<Tab>, tx: Sender<String>, item: SelectedBotItem) -> Result<(), failure::Error>{
    // size=64905&style=29235&qty=1
    // let url = format!("https://www.supremenewyork.com/shop/{}/add.json", item.id);
    // let post_body = format!("size={}&style={}&qty=1", item.size_id, item.style_id);
    // let response = client.post(&url).body(post_body).send().await?;
    // let set_cookie_iter = response.headers().get_all(SET_COOKIE);
    // let mut cookies: Vec<SetCookie> = Vec::new();
    // for c in set_cookie_iter {
    //     c.to_str()
    //         .into_iter()
    //         .map(|s| s.to_string())
    //         .for_each(|cookie_str| {
    //             Cookie::parse(cookie_str).into_iter().for_each(|c| {
    //                 //println!("{:?}", c);
    //                 let mut domain = "www.supremenewyork.com";
    //                 let mut secure = if c.name() == "_state" {false} else {true};
    //                 let mut expires = if c.name() == "_state" {Option::from(1000000000.0)} else {None};
    //                 let cookie = SetCookie{
    //                     name: c.name().to_string(),
    //                     value: c.value().to_string(),
    //                     url: Option::from("https://www.supremenewyork.com/".to_string()),
    //                     domain: Option::from(domain.to_string()),
    //                     path: Option::from("/".to_string()),
    //                     secure: Option::from(secure),
    //                     http_only: c.http_only(),
    //                     same_site: None,
    //                     expires: expires,
    //                     priority: Option::from(CookiePriority::High)
    //                 };
    //                 cookies.push(cookie);
    //             });
    //         });
    // }
    // let response_txt = response.text().await?;
    // //Looks like if the item is sold out it returns []
    // if response_txt != "[]"{
    //     tx.send(format!("Successfully added to cart")).expect("Couldn't send data to channel");
    //     println!("Added to cart - {}", response_txt);
    // }

    // tab.navigate_to(&format!("https://www.supremenewyork.com/"))?;
    // for cookie in &cookies {
    //     println!("cart | {:?}", &cookie);
    // }

    //tab.set_cookies(cookies)?;
    // let req_pattern = RequestPattern{ url_pattern: Option::from(""), resource_type: Option::from("Document"), interception_stage: Option::from("Request")};
    // let req_interceptor: RequestInterceptor = Box::new(|transport: Arc<_>, session_id, params| {
    //     println!("req_interceptor: {:?}", params);
    //     Continue
    // });
    //tab.enable_request_interception(&[req_pattern], req_interceptor)?;
    let mut sw = Stopwatch::start_new();
    tab.wait_until_navigated().ok();
    tab.navigate_to(&format!("https://www.supremenewyork.com/mobile/#products/{}/{}", item.id, item.style_id));
    println!("navigating to product");
    let eval_expression = format!("$(\"#size-options\").val({});$(\"#size-options\").change();productDetailView.addToCartButton.addToCart()", item.size_id);
    //println!("{}", ajax_req);
    sleep(std::time::Duration::from_millis(1400));
    tab.evaluate(&eval_expression, true).ok();
    // let mut real_cookies = tab.get_cookies()?;
    // //client.get_cookies(&mut real_cookies);
    // for cookie in real_cookies {
    //     println!("real | {:?}", cookie);
    // }
    tx.send(format!("Add to cart: {}", sw.elapsed_ms()));
    Ok(())
}

#[tokio::main]
pub async fn wait_for_drop(tx: Sender<String>) -> Result<(), reqwest::Error>{
    let url = "https://www.supremenewyork.com/mobile_stock.json";
    let mut old_items: Vec<i64> = Vec::new();
    let mut new_items: Vec<i64> = Vec::new();
    loop {
        let body = reqwest::get(url)
            .await?
            .text()
            .await?;
        let json: serde_json::Value = serde_json::from_str(&body).expect("JSON was not well-formatted");
        for cat_item_list in json["products_and_categories"].as_object().unwrap().values() {
            for item_list in cat_item_list.as_array().iter() {
                for item in item_list.iter() {
                    old_items.push(item["id"].as_i64().unwrap());
                }
            }
        }
        if new_items.len() > 0 {
            let matching = old_items.iter().zip(&new_items).filter(|&(a, b)| a == b).count();
            if matching != new_items.len(){
                let now = Local::now();
                tx.send(format!("Drop detected at {}", now.format("%H:%M:%S%.1f"))).expect("Couldn't send data to channel");
                break;
            }
        } else {
            new_items = old_items.clone();
        }
        let sleep_dur = std::time::Duration::from_millis(200);
        sleep(sleep_dur);
    }
    Ok(())
}

struct CheckoutParams{
    name: String,
    email: String,
    tel: String,
    address: String,
    city: String,
    zip: String,
}

#[tokio::main]
pub async fn checkout(client: &mut reqwest::Client, tab: Arc<Tab>, item: SelectedBotItem, shipping_profile: Profile, card_profile: CreditCard) -> Result<String, failure::Error>{

    sleep(std::time::Duration::from_millis(400));
    tab.evaluate("Supreme.app.checkout()", true)?;
    tab.wait_until_navigated();

    let select_fields = tab.wait_for_elements(".selectric-wrapper")?;
    for select in select_fields {

        let field_attributes = match select.get_attributes()? {
            Some(attributes) => attributes,
            None => HashMap::new() as NodeAttributes
        };
        if !field_attributes.is_empty() {

            let node = select.get_description()?;
            let children = match node.children {
                Some(nodes) => nodes,
                None => Vec::new()
            };
            let child_len = children.clone().len();
            println!("{}, {:?}", child_len, children);
            let mut field_name: String = "None".to_string();
            if child_len == 4 {
                for (i, node) in children.iter().enumerate() {
                    let node_clone = node.clone();
                    let node_attributes = match node_clone.attributes {
                        Some(attributes) => attributes,
                        None => HashMap::new() as NodeAttributes
                    };
                    println!("node {}, {:?}", i, node_attributes);

                    if i == 0 {
                        let mut selectric_select = node_clone.children.unwrap();
                        if selectric_select.clone().len() == 1 {
                            let element = selectric_select.get(0).unwrap();
                            let node_attributes = match element.clone().attributes {
                                Some(attributes) => attributes,
                                None => HashMap::new() as NodeAttributes
                            };
                            field_name = node_attributes.get("name").unwrap().to_string();
                            println!("added {}", field_name);
                        }
                    }
                }

                if field_name.contains("country") {
                    select.type_into(&shipping_profile.country)?.click()?;
                }
                else if field_name.contains("type") {
                    select.type_into(&card_profile._type)?.click()?;
                }
                else if field_name.contains("month") {
                    select.type_into(&card_profile.month)?.click()?;
                }
                else if field_name.contains("year") {
                    select.type_into(&card_profile.year)?.click()?;
                }
            }
        }

    }

    let input_fields = tab.wait_for_elements("input")?;
    for input in input_fields {
        let field_attr = match input.get_attributes()? {
            Some(attr) => attr,
            None => HashMap::new() as NodeAttributes
        };
        if !field_attr.is_empty() {
            match field_attr.get("type") {
                Some(_type) => {
                    let field_name = match field_attr.get("name") {
                        Some(name) => {
                            println!("{}, #{}", field_attr.get("type").unwrap(), name);
                            name.to_string()
                        }
                        None => {
                            println!("{}, no id found", field_attr.get("type").unwrap());
                            "no-name".to_string()
                        }
                    };
                    if _type == "text" {
                        if field_name.contains("name") {
                            legit_type_into(input, &shipping_profile.full_name);
                            //input.type_into(&shipping_profile.full_name).ok().expect("Couldn't write to input");
                        } else if field_name.contains("address") && !field_name.contains("2") && !field_name.contains("3") {
                            legit_type_into(input, &shipping_profile.address);
                            //input.type_into(&shipping_profile.address).ok().expect("Couldn't write to input");
                        } else if field_name.contains("city") {
                            legit_type_into(input, &shipping_profile.city);
                            //input.type_into(&shipping_profile.city).ok().expect("Couldn't write to input");
                        } else if field_name.contains("zip") {
                            legit_type_into(input, &shipping_profile.postcode);
                            //input.type_into(&shipping_profile.postcode).ok().expect("Couldn't write to input");
                        } else {}
                    }
                    else if _type == "tel" {
                        if field_name.contains("cnb") || field_name.contains("number") {
                            legit_type_into(input, &card_profile.number);
                            //input.type_into(&card_profile.number).ok().expect("Couldn't write to input");
                        } else if field_name.contains("vv") {
                            legit_type_into(input, &card_profile.cvv);
                            //input.type_into(&card_profile.cvv).ok().expect("Couldn't write to input");
                        } else {
                            legit_type_into(input, &shipping_profile.tel);
                            //input.type_into(&shipping_profile.tel).ok().expect("Couldn't write to input");
                        }
                    }
                    else if _type == "email" {
                        legit_type_into(input, &shipping_profile.email);
                        //input.type_into(&shipping_profile.email).ok().expect("Couldn't write to input");
                    }
                },
                None => ()
            }
        }

    }
    tab.wait_for_element("input#order_terms")?.click()?;
    sleep(std::time::Duration::from_millis(4000));
    tab.wait_for_element("button[type]")?.click()?;
    let current_url = tab.get_url();
    while tab.get_url() == current_url {};
    let charge_status = crop_letters(&mut tab.get_url(), 39).to_string();
    Ok(charge_status)
}

#[tokio::main]
pub async fn get_order_status(client: &mut reqwest::Client, order_id: i64) -> Result<(String), reqwest::Error>{
    let order_url = format!("https://www.supremenewyork.com/checkout/{}/status.json", order_id);
    let mut status = "queued".to_string();
    let mut retries_count = 1;
    while status == "queued" && retries_count <= 10 {

        let response = client.get(&order_url).send().await?.text().await?;
        let json: serde_json::Value = serde_json::from_str(&response).expect("JSON was not well-formatted");
        let previous_status = status;
        status = match json["status"].as_str() {
            Some(_status) => _status.to_string(),
            None => "no status".to_string()
        };
        sleep(std::time::Duration::from_millis(400));
        retries_count += 1;

    }
    if status == "queued"{
        status = "failed - queued too many times".to_string();
    }
    Ok((status))
}

#[tokio::main]
pub async fn parse_checkout_parameters(client: &mut reqwest::Client, checkout_headers: reqwest::header::HeaderMap ) -> Result<(), failure::Error>{
    // let browser = Browser::new(LaunchOptions::default_builder().headless(false).build().unwrap()).ok().unwrap();
    // let tab = browser.wait_for_initial_tab()?;
    // tab.set_cookies();
    // tab.navigate_to("https://www.supremenewyork.com/mobile/#checkout")?;
    // sleep(std::time::Duration::from_secs(5));
    // let response: String = client.get("https://www.supremenewyork.com/mobile/#checkout").headers(checkout_headers).send().await?.text().await?;
    //
    // let soup = Soup::new(&response);
    // write_to_debug(soup.text());
    // for form in soup.tag("form").find_all() {
    //     println!("{}", form.text());
    // }
    // let mut input_list: Vec<String> = Vec::new();
    //
    // for (i, input) in soup.tag("input").find_all().enumerate() {
    //     let name = match input.get("name"){
    //         Some(name) => name,
    //         None => "".to_string()
    //     };
    //     input_list.push(name);
    // }
    //
    // for input in input_list{
    //     println!("Input: {}",input);
    // }
    Ok(())
}
// let checkout_body = format!("store_credit_id=&\
// from_mobile=1&\
// cookie-sub={}&\
// cardinal_id=1_e4ba3c00-3241-486f-91ce-2ae778d992cc&\
// same_as_billing_address=1&\
// order%5Bbilling_name%5D=aaaaa+aldo&\
// order%5Bemail%5D=aaaaaaaaa%40hotmail.com&\
// order%5Btel%5D=1111111111&\join_mailinglist
// %5Bbilling_address%5D=via+aldo&\
// order%5Bbilling_address_2%5D=&\
// order%5Bbilling_address_3%5D=&\
// order%5Bbilling_city%5D=to+soru&\
// atok=sckrsarur&\
// order%5Bbilling_zip%5D=45030&\
// order%5Bbilling_country%5D=IT&\
// credit_card%5Btype%5D=visa&\
// credit_card%5Bcnb%5D=4124+5764+0586+3608&\
// credit_card%5Bmonth%5D=09&\
// credit_card%5Byear%5D=2020&\
// credit_card%5Bovv%5D=123&\
// order%5Bterms%5D=0&\
// order%5Bterms%5D=1&\
// g-recaptcha-response=03AGdBq240PYDuP1tjs1ovqD0C8O1ZKCZ3GcynGG-RAoIh124nOyEv9YZwJdONe0YAkwUFKIpmmfCwK7OCoSvRfhyo1sijxX8DvFLa3npONn4oRqKyWgHT_8agcuOgr0DXletE7q7dgxAyzHMjD2GT0lZh5vXQnq1DffnWHmzRq0zHmFpPhByKfKxRqdxIW6rvDGW4g8AVjL9QtKfTlcgZs5t9hBVIbU4ocCYkm6S7GgeHm11AReFNKvpx5Yda62Wl5sydGWM_ycsaQvIDeSI0hNapEA3ubk4Sy9T0U88R9zzly3edCAAPvhSlB25-yR4H4bYE9jzKe_FXO7fLrVU5AbD_37GBZIwtbz7szUKdPv1RrkWhjAVal3xTpGfM2EkOP9kXQuhA8D3ev1DsGfHeHGCz6Sd24Oy4sA",
// encoded_cookie_sub);

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

// #[tokio::main]
// println!("'{}' , '{}' ", shipping_profile.country, shipping_profile.postcode);
// let body = form_urlencoded::Serializer::new(String::new())
//     .append_pair("store_credit_id", "")
//     .append_pair("from_mobile", "1")
//     .append_pair("cookie-sub", &format!("%7B%22{}%22%3A1%7D", item.style_id))
//     .append_pair("cardinal_id", "1_e4ba3c00-3241-486f-91ce-2ae778d992cc")
//     .append_pair("same_as_billing_address", "1")
//     .append_pair("order[billing_name]", &shipping_profile.full_name)
//     .append_pair("oinput[type=hidden]rder[email]", &shipping_profile.email)
//     .append_pair("order[tel]", &shipping_profile.tel)
//     .append_pair("order[billing_address]", &shipping_profile.address)
//     .append_pair("order[billing_address_2]", "")
//     .append_pair("order[billing_address_3]", "")
//     .append_pair("order[billing_city]", &shipping_profile.city)
//     .append_pair("atok",form input "sckrsarur")
//     .append_pair("order[billing_country]", &shipping_profile.country)
//     .append_pair("order[billing_zip]", &shipping_profile.postcode)
//     .append_pair("credit_card[brand]", &card_profile._type)
//     .append_pair("credit_card[number]", &card_profile.number)
//     .append_pair("credit_card[month]", &card_profile.month)
//     .append_pair("credit_card[year]", &card_profile.year)
//     .append_pair("credit_card[ovv]", &card_profile.cvv)
//     .append_pair("order[terms]", "0")
//     .append_pair("order[terms]", "1")
//     .append_pair("g-recaptcha-response", "03AGdBq240PYDuP1tjs1ovqD0C8O1ZKCZ3GcynGG-RAoIh124nOyEv9YZwJdONe0YAkwUFKIpmmfCwK7OCoSvRfhyo1sijxX8DvFLa3npONn4oRqKyWgHT_8agcuOgr0DXletE7q7dgxAyzHMjD2GT0lZh5vXQnq1DffnWHmzRq0zHmFpPhByKfKxRqdxIW6rvDGW4g8AVjL9QtKfTlcgZs5t9hBVIbU4ocCYkm6S7GgeHm11AReFNKvpx5Yda62Wl5sydGWM_ycsaQvIDeSI0hNapEA3ubk4Sy9T0U88R9zzly3edCAAPvhSlB25-yR4H4bYE9jzKe_FXO7fLrVU5AbD_37GBZIwtbz7szUKdPv1RrkWhjAVal3xTpGfM2EkOP9kXQuhA8D3ev1DsGfHeHGCz6Sd24Oy4sA")
// .finish();
// let response = client.post("https://www.supremenewyork.com/checkout.json").body(body).send().await?.text().await?;
// let json: serde_json::Value = serde_json::from_str(&response).expect("JSON was not well-formatted");
// let order_id = match json["id"].as_i64() {
//     Some(id) => id,
//     None => 0
// };