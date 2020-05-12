use crate::bot;
use stopwatch::Stopwatch;
use std::thread::sleep;
use std::time;
use crate::req_handler;
use crate::bot_utils;
use crate::bot_utils::{ShopItem, SelectedBotItem};
use reqwest::ClientBuilder;
use reqwest::header;

#[allow(dead_code)]
pub fn start_benchmark(times_to_run: i64){

    // Bot stuff
    let mut headers = header::HeaderMap::new();
    bot_utils::gen_header_map(&mut headers);

    let mut client = ClientBuilder::new()
        .default_headers(headers)
        .cookie_store(true)
        .build().unwrap();

    let mut shop_item_array: Vec<ShopItem> = Vec::new();
    let mut sw = Stopwatch::new();
    let mut measurement_vec: i64 = 0;
    let sleep_duration = time::Duration::from_millis(2000);

    println!("Starting benchmark...");
    for _i in 0..times_to_run {
        sw.start();
        bot_sim("dog top".to_string(), "black".to_string(), "Any".to_string(), &mut shop_item_array, &mut client);
        let elapsed = sw.elapsed_ms();
        measurement_vec += elapsed;
        sleep(sleep_duration);
        sw.reset();
    }
    println!("Avg speed: {}ms", measurement_vec/times_to_run);
}

#[allow(dead_code)]
fn bot_sim(item_kw: String, item_color: String, sel_size: String, mut shop_item_array: &mut Vec<ShopItem>, mut client: &mut reqwest::Client){

    // Populate array
    req_handler::fetch_shop_items(&mut shop_item_array).ok();

    // Find the item
    let item = bot::find_requested_item(&mut client, &mut shop_item_array, item_kw, item_color, sel_size);

    if item.status > 0 {
        if item.status == 1 {
            println!("Unable to find item :/");
        } else if item.status == 2{
            println!("Found {} ({})", item.name, item.color);
            println!("Item is sold-out :(");
        } else {}
    } else {
        add_to_cart(&mut client, item).ok();
    }

}

#[allow(dead_code)]
#[tokio::main]
async fn add_to_cart(client: &mut reqwest::Client, item: SelectedBotItem) -> Result<(), reqwest::Error>{
    // size=64905&style=29235&qty=1
    let url = format!("https://www.supremenewyork.com/shop/{}/add.json", item.id);
    let post_body = format!("size={}&style={}&qty=1", item.size_id, item.style_id);
    let _response = client.post(&url).body(post_body).send().await?.text().await?;
    // Looks like if the item is sold out it returns []

    Ok(())
}