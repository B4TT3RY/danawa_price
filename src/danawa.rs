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

#[derive(Debug)]
pub enum SearchError {
    Http(reqwest::Error),
    Server(reqwest::StatusCode),
    Parse(unhtml::Error),
}

impl Searcher {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub async fn get_product_info(&self, product_code: &str) -> Result<ProductInfo, SearchError> {
        let url = format!("{}{}", self.url, product_code);
        let res = reqwest::get(&url).await?;
        if !res.status().is_success() {
            return Err(SearchError::Server(res.status()));
        }
        assert!(res.status().is_success());

        let body = res.text().await?;
        let data = DanawaData::from_html(&body)?;

        let card_price = data.card_price.and_then(|price| price.replace(",", "").parse().ok());
        let cash_price = data.cash_price.and_then(|price| price.replace(",", "").parse().ok());

        Ok(ProductInfo {
            product_name: data.product_name,
            url,
            price: PriceData {
                card_price,
                cash_price,
            },
        })
    }
}

impl From<reqwest::Error> for SearchError {
    fn from(e: reqwest::Error) -> Self {
        Self::Http(e)
    }
}

impl From<unhtml::Error> for SearchError {
    fn from(e: unhtml::Error) -> Self {
        Self::Parse(e)
    }
}

impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(e) => write!(f, "Error sending http request: {}", e),
            Self::Server(status) => write!(f, "Server returned bad status: {}", status),
            Self::Parse(e) => write!(f, "Error parsing danawa: {}", e),
        }
    }
}

impl std::error::Error for SearchError {}
