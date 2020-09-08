extern crate unhtml;
extern crate serde;
extern crate reqwest;
extern crate config;
extern crate toml;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use unhtml::FromHtml;
use num_format::{Locale, ToFormattedString};

mod settings;
use settings::Settings;

#[derive(FromHtml, Debug)]
#[html(selector = "#danawa_container")]
struct DanawaData {
    #[html(selector = ".prod_tit", attr = "inner")]
    product_name: String,

    #[html(selector = ".lowest_price .prc_c", attr = "inner", default = "정보없음")]
    card_price: String,

    #[html(selector = "#lowPriceCash .prc_c", attr = "inner", default = "정보없음")]
    cash_price: String,
}

#[tokio::main]
async fn main() {
    let settings = Settings::new().unwrap();
    
    let mut file = OpenOptions::new()
                            .create(true)
                            .read(true)
                            .write(true)
                            .open("Data.toml")
                            .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut prev_price_map: HashMap<String, String> = toml::from_str::<HashMap<String, String>>(&contents).unwrap();

    for product_code in settings.danawa.product_list.iter() {
        let res = danawa(&settings, &product_code).await;
        let prev_card_price = prev_price_map.get(&format!("card_{}", product_code)).cloned().unwrap_or_else(|| String::from("정보없음"));
        let prev_cash_price = prev_price_map.get(&format!("cash_{}", product_code)).cloned().unwrap_or_else(|| String::from("정보없음"));
        println!("{} ({})", res.product_name, format!("{}{}", settings.danawa.url, product_code));
        println!(" - 카드가: {}원 ({})", res.card_price, price_distance(&prev_card_price, &res.card_price));
        println!(" - 현금가: {}원 ({})", res.cash_price, price_distance(&prev_cash_price, &res.cash_price));

        prev_price_map.insert(format!("card_{}", product_code), res.card_price);
        prev_price_map.insert(format!("cash_{}", product_code), res.cash_price);
    }

    file = OpenOptions::new()
                .write(true)
                .append(false)
                .open("Data.toml")
                .unwrap();
    file.write_all(toml::to_string(&prev_price_map).unwrap().as_bytes()).unwrap();
    file.flush().unwrap();
}

async fn danawa(settings: &Settings, product_code: &str) -> DanawaData {
    let url = format!("{}{}", settings.danawa.url, product_code);
    let res = reqwest::get(&url).await.unwrap();
    assert!(res.status().is_success());

    let body = res.text().await.unwrap();
    DanawaData::from_html(&body).unwrap()
}

fn price_distance(prev: &str, now: &str) -> String {
    if now == "정보없음" {
        return String::from("-");
    }

    let prev = if prev == "정보없음" {
        0
    } else {
        prev.replace(",", "").parse::<i32>().unwrap()
    };
    let now = now.replace(",", "").parse::<i32>().unwrap();

    let distance = now - prev;
    if distance > 0 {
        format!("▲{}원", distance.to_formatted_string(&Locale::ko))
    } else if distance < 0 {
        format!("▼{}원", distance.abs().to_formatted_string(&Locale::ko))
    } else {
        String::from("-")
    }
}