use unhtml::FromHtml;
use unhtml_derive::FromHtml;

use crate::price::PriceData;

#[derive(FromHtml, Debug)]
#[html(selector = "#danawa_container")]
struct DanawaData {
    #[html(selector = ".prod_tit", attr = "inner")]
    product_name: String,

    #[html(selector = ".lowest_price .prc_c", attr = "inner")]
    card_price: Option<String>,

    #[html(selector = "#lowPriceCash .prc_c", attr = "inner")]
    cash_price: Option<String>,
}

pub struct ProductInfo {
    pub product_name: String,
    pub url: String,
    pub price: PriceData,
}

pub struct Searcher {
    url: String,
}

impl Searcher {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub async fn get_product_info(&self, product_code: &str) -> ProductInfo {
        let url = format!("{}{}", self.url, product_code);
        let res = reqwest::get(&url).await.unwrap();
        assert!(res.status().is_success());

        let body = res.text().await.unwrap();
        let data = DanawaData::from_html(&body).unwrap();

        let card_price = data.card_price.map(|price| price.parse().unwrap());
        let cash_price = data.cash_price.map(|price| price.parse().unwrap());

        ProductInfo {
            product_name: data.product_name,
            url,
            price: PriceData {
                card_price,
                cash_price,
            },
        }
    }
}
