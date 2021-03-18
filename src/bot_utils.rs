
use std::string::String;
use std::borrow::BorrowMut;
use std::str::SplitWhitespace;
use reqwest::header;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, CONTENT_TYPE, ORIGIN, REFERER, CONNECTION, DNT, USER_AGENT, PRAGMA,TE,CACHE_CONTROL, UPGRADE_INSECURE_REQUESTS};
use crate::profileHandler::{Profile, CreditCard};
use headless_chrome::browser::tab::element::Element;
use std::thread::sleep;
use headless_chrome::Tab;
use std::sync::Arc;
use rand::distributions::{Distribution, Uniform};

#[derive(Clone,Debug)]
pub struct ShopItem{
    pub name: String,
    pub id: i64,
    pub matched_item_keywords: i8
}

impl Default for ShopItem{
    fn default() -> ShopItem {
        ShopItem {
            name: "".to_string(),
            id: 0,
            matched_item_keywords: 0
        }
    }
}

// status == 1 : Not found
// status == 2 : All sizes are sold out

#[derive(Clone, Debug)]
pub struct SelectedBotItem{
    pub name: String,
    pub color: String,
    pub size: String,
    pub id: i64,
    pub style_id: i64,
    pub size_id: i64,
    pub status: i16
}

#[derive(Clone, Debug)]
pub struct BotItem{
    pub name: String,
    pub color: String,
    pub matched_item_keywords: i8,
    pub matched_color_keywords: i8,
    pub id: i64,
    pub style_id: i64,
    pub sizes: Vec<ItemSize>
}

#[derive(Clone, Debug)]
pub struct ItemSize{
    pub name: String,
    pub id: i64,
    pub in_stock: bool
}

impl Default for SelectedBotItem{
    fn default() -> SelectedBotItem{
        SelectedBotItem{
            name: "".to_string(),
            color: "".to_string(),
            size: "".to_string(),
            id: 0,
            style_id: 0,
            size_id: 0,
            status: 0
        }
    }
}

impl Default for BotItem {
    fn default () -> BotItem {
        BotItem{
            name: "".to_string(),
            color: "".to_string(),
            matched_item_keywords: 0,
            matched_color_keywords: 0,
            id: 0,
            style_id: 0,
            sizes: Vec::new()
        }
    }
}

pub fn gen_scraping_header_map(headers: &mut header::HeaderMap){
    let user_agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 13_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/80.0.3987.95 Mobile/15E148 Safari/604.1";
    headers.insert(USER_AGENT, header::HeaderValue::from_str(user_agent).unwrap());
    headers.insert(ACCEPT, header::HeaderValue::from_str("application/json").unwrap());
    headers.insert(ACCEPT_LANGUAGE, header::HeaderValue::from_str("en-US,en;q=0.5").unwrap());
    headers.insert("X-Requested-With", header::HeaderValue::from_str("XMLHttpRequest").unwrap());
    headers.insert(CONTENT_TYPE, header::HeaderValue::from_str("application/x-www-form-urlencoded").unwrap());
    headers.insert(ORIGIN, header::HeaderValue::from_str("https://www.supremenewyork.com").unwrap());
    headers.insert(DNT, header::HeaderValue::from_str("1").unwrap());
    headers.insert(CONNECTION, header::HeaderValue::from_str("keep-alive").unwrap());
    headers.insert(REFERER, header::HeaderValue::from_str("https://www.supremenewyork.com/mobile/").unwrap());
    headers.insert(UPGRADE_INSECURE_REQUESTS, header::HeaderValue::from_str("1").unwrap());
    headers.insert(PRAGMA, header::HeaderValue::from_str("no-cache").unwrap());
    headers.insert(CACHE_CONTROL, header::HeaderValue::from_str("no-cache").unwrap());
    headers.insert(TE, header::HeaderValue::from_str("Trailers").unwrap());

}

pub fn gen_checkout_header_map(headers: &mut header::HeaderMap){
    let user_agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 13_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/80.0.3987.95 Mobile/15E148 Safari/604.1";
    headers.insert(USER_AGENT, header::HeaderValue::from_str(user_agent).unwrap());
    headers.insert(ACCEPT, header::HeaderValue::from_str("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8").unwrap());
    headers.insert(ACCEPT_LANGUAGE, header::HeaderValue::from_str("en-US,en;q=0.5").unwrap());
    headers.insert(DNT, header::HeaderValue::from_str("1").unwrap());
    headers.insert(CONNECTION, header::HeaderValue::from_str("keep-alive").unwrap());
    headers.insert(UPGRADE_INSECURE_REQUESTS, header::HeaderValue::from_str("1").unwrap());
    headers.insert(PRAGMA, header::HeaderValue::from_str("no-cache").unwrap());
    headers.insert(CACHE_CONTROL, header::HeaderValue::from_str("no-cache").unwrap());
    headers.insert(TE, header::HeaderValue::from_str("Trailers").unwrap());

}

pub fn find_available_size(item: &mut BotItem, sel_size: String) -> SelectedBotItem{

    let mut sel_item = SelectedBotItem::default();
    for size in item.sizes.iter(){
        if size.in_stock{
            if sel_size == "Any" || sel_size.to_lowercase() == size.name.to_lowercase(){
                sel_item.name = item.name.to_owned();
                sel_item.color = item.color.to_owned();
                sel_item.size = size.name.to_owned();
                sel_item.id = item.id;
                sel_item.style_id = item.style_id;
                sel_item.size_id = size.id;
                sel_item.status = 0;
                break;
            }
        } else {
            sel_item.name = item.name.to_owned();
            sel_item.color = item.color.to_owned();
            sel_item.status = 2;
        }
    }

    return sel_item;
}

pub fn find_most_likely_item(shop_item: ShopItem, best_match: &mut ShopItem){

    if shop_item.matched_item_keywords >= best_match.matched_item_keywords {
        *best_match = shop_item;
    }

}

pub fn find_most_likely_color(shop_item: BotItem, best_match: &mut BotItem){

    if shop_item.matched_color_keywords >= best_match.matched_color_keywords {
        *best_match = shop_item;
    }
}

pub fn compare_name_keywords(shop_item: &mut ShopItem, user_item: String){

    let mut item_string = shop_item.name.clone().to_lowercase();
    remove_whitespace(item_string.borrow_mut());
    let user_item_clone = user_item.clone().to_lowercase();
    let user_item_kw = user_item_clone.split_whitespace();
    let mut item_kw_found: i8 = 0;

    compare_kw(user_item_kw, item_string, &mut item_kw_found);

    shop_item.matched_item_keywords = item_kw_found;

}

pub fn compare_color_keywords(shop_item: &mut BotItem, user_color: String){

    let mut color_string = shop_item.color.clone().to_lowercase();
    remove_whitespace(color_string.borrow_mut());
    let user_color_clone = user_color.clone().to_lowercase();
    let user_color_kw = user_color_clone.split_whitespace();
    let mut color_kw_found: i8 = 0;

    compare_kw(user_color_kw, color_string, &mut color_kw_found);

    shop_item.matched_color_keywords = color_kw_found;

}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn compare_kw(mut keywords: SplitWhitespace, shop_item: String, kw_found: &mut i8){

    let kw_left = keywords.clone().count();
    let current_kw = keywords.next();
    if kw_left > 0 {
        let current_kw = current_kw.unwrap();
        //println!("Kw_left: {}, current_kw: {}, shop_str: {}", kw_left, current_kw, shop_item);
        //println!("{}",shop_item.contains(current_kw));

        if shop_item.contains(current_kw) {
            *kw_found += 1;
            //println!("Found, kw_found: {}", kw_found);
        } else {
            //println!("Keyword not found: {}", current_kw);
        }
        compare_kw(keywords.clone(), shop_item, kw_found);

    } else {
        //println!("Finished keywords");
        return;
    }
}

pub fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}

pub fn legit_type_into(input: Element, string: &str){

    for i in 0..string.len(){
        input.type_into(&string.chars().nth(i).unwrap().to_string()).ok().expect("Couldn't write to input");
        let step = Uniform::new(80, 110);
        let mut rng = rand::thread_rng();
        let choice = step.sample(&mut rng) as u64;
        sleep(std::time::Duration::from_millis(choice));
    }

}

pub fn legit_type_str(input: Arc<Tab>, string: &str){

    for i in 0..string.len(){
        input.type_str(&string.chars().nth(i).unwrap().to_string());
        let step = Uniform::new(80, 110);
        let mut rng = rand::thread_rng();
        let choice = step.sample(&mut rng) as u64;
        sleep(std::time::Duration::from_millis(choice));
    }

}