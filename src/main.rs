use chrono::prelude::*;
use num_format::{Locale, ToFormattedString};

mod danawa;
mod price;
mod settings;
mod telegram;

use price::PriceData;
use settings::Settings;
use telegram::escape;

#[tokio::main]
async fn main() {
    let local: DateTime<Local> = Local::now();

    let settings = Settings::new().expect("Error reading settings");

    let mut price_map =
        price::PriceStorage::load("Data.toml").unwrap_or_else(|_| Default::default());
    let tg_client = telegram::Sender::new(&settings.telegram.bot_token, &settings.telegram.chat_id);
    let searcher = danawa::Searcher::new(&settings.danawa.url);

    let mut message = String::new();
    let mut max_difference: Vec<i32> = Vec::new();
    for product_code in settings.danawa.product_list {
        let res = searcher
            .get_product_info(&product_code)
            .await
            .expect("Error getting product info");
        let PriceData {
            card_price,
            cash_price,
        } = res.price;

        let PriceData {
            card_price: prev_card_price,
            cash_price: prev_cash_price,
        } = price_map.get(&product_code);

        if prev_card_price != card_price || prev_cash_price != cash_price {
            let card_diff = price_difference(prev_card_price, card_price);
            let cash_diff = price_difference(prev_cash_price, cash_price);

            let card_price =
                card_price.map_or("정보없음".to_string(), |price| format!("{}원", price.to_formatted_string(&Locale::ko)));
            let cash_price =
                cash_price.map_or("정보없음".to_string(), |price| format!("{}원", price.to_formatted_string(&Locale::ko)));

            let new_content = format!(
                "[{product_name}]({product_url})%0D%0A\
                `\\- 카드가: {card_price} ({card_diff})`%0D%0A\
                `\\- 현금가: {cash_price} ({cash_diff})`%0D%0A",
                product_name = escape(&res.product_name),
                product_url = res.url,
                card_price = card_price,
                card_diff = card_diff,
                cash_price = cash_price,
                cash_diff = cash_diff,
            );
            message.push_str(&new_content);

            max_difference.push(match card_diff {
                Difference::Up(up) => up,
                Difference::Down(down) => down,
                _ => 0,
            });

            max_difference.push(match cash_diff {
                Difference::Up(up) => up,
                Difference::Down(down) => down,
                _ => 0,
            });
        }

        price_map.insert(
            product_code,
            PriceData {
                card_price,
                cash_price,
            },
        );
    }

    if !message.is_empty() {
        let disable_notification = max_difference.iter().max().unwrap_or(&0) < &1000;
        tg_client
            .send_message(&message, disable_notification)
            .await
            .expect("Error sending message");
    }

    price_map
        .save("Data.toml")
        .expect("Error saving price data");

    if settings.telegram.update_chat_description {
        tg_client
            .set_chat_description(&format!(
                "마지막 확인: {}",
                local.format("%Y년 %m월 %d일 %H시 %M분")
            ))
            .await
            .expect("Error setting chat description");
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
