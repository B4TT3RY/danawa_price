#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
extern crate serde;
extern crate reqwest;
extern crate config;
extern crate toml;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use num_format::{Locale, ToFormattedString};
use chrono::prelude::*;

mod settings;
mod telegram;
mod danawa;
use settings::Settings;
use telegram::syntax;

#[tokio::main]
async fn main() {
    let local: DateTime<Local> = Local::now();

    let settings = Settings::new().unwrap();

    let mut file = OpenOptions::new()
                            .create(true)
                            .read(true)
                            .write(true)
                            .open("Data.toml")
                            .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut prev_price_map = toml::from_str::<HashMap<String, i32>>(&contents).unwrap();
    let mut message = String::new();
    let tg_client = telegram::Sender::new(&settings.telegram.bot_token, &settings.telegram.chat_id);
    let searcher = danawa::Searcher::new(&settings.danawa.url);

    for product_code in settings.danawa.product_list.iter() {
        let res = searcher.get_product_info(&product_code).await;

        let prev_card_price = prev_price_map.get(&format!("card_{}", product_code)).copied();
        let prev_cash_price = prev_price_map.get(&format!("cash_{}", product_code)).copied();

        if prev_card_price != res.card_price || prev_cash_price != res.cash_price {
            let card_diff = price_difference(prev_card_price, res.card_price);
            let cash_diff = price_difference(prev_cash_price, res.cash_price);

            let card_price = res.card_price.map_or("정보없음".to_string(), |price| format!("{}원", price));
            let cash_price = res.cash_price.map_or("정보없음".to_string(), |price| format!("{}원", price));

            let new_content = format!(
                "[{product_name}]({product_url})%0D%0A\
                `\\- 카드가: {card_price}원 ({card_diff})`%0D%0A\
                `\\- 현금가: {cash_price}원 ({cash_diff})`%0D%0A",
                product_name = syntax(&res.product_name),
                product_url = res.url,
                card_price = card_price,
                card_diff = card_diff,
                cash_price = cash_price,
                cash_diff = cash_diff,
            );
            message.push_str(&new_content);
        }

        if let Some(card_price) = res.card_price {
            prev_price_map.insert(format!("card_{}", product_code), card_price);
        }
        if let Some(cash_price) = res.cash_price {
            prev_price_map.insert(format!("cash_{}", product_code), cash_price);
        }
    }

    if !message.is_empty() {
        tg_client.send_message(&message).await;
    }

    file = OpenOptions::new()
                .write(true)
                .append(false)
                .open("Data.toml")
                .unwrap();
    file.write_all(toml::to_string(&prev_price_map).unwrap().as_bytes()).unwrap();
    file.flush().unwrap();

    if settings.telegram.update_chat_description {
        tg_client.send_message(&format!("마지막 확인: {}", local.format("%Y년 %m월 %d일 %H시 %M분"))).await;
    }
}

enum Difference {
    Error,
    Up(i32),
    Stay,
    Down(i32),
}

impl std::fmt::Display for Difference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difference::Error => f.write_str("정보없음"),
            Difference::Stay => f.write_str("-"),
            Difference::Up(up) => write!(f, "▲{}원", up.to_formatted_string(&Locale::ko)),
            Difference::Down(down) => write!(f, "▼{}원", down.to_formatted_string(&Locale::ko)),
        }
    }
}

fn price_difference(prev: Option<i32>, now: Option<i32>) -> Difference {
    match (prev, now) {
        (_, None) => Difference::Stay,
        (None, _) => Difference::Error,
        (Some(prev), Some(now)) => {
            let diff = now - prev;
            if diff > 0 {
                Difference::Up(diff)
            } else if diff < 0 {
                Difference::Down(-diff)
            } else {
                Difference::Stay
            }
        }
    }
}