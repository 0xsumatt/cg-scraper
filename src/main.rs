
use serde::{Deserialize,Serialize};
use reqwest::blocking::get;



fn main() {
  let res = get("https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=250&page2&sparkline=false").unwrap();
  let tokens = res.json::<Response>().unwrap();
  for token in tokens
    {
      
        let ts = match token.total_supply{
          Some (ts) => ts as f64,
          None => 0.0
        };

        let cs = token.circulating_supply;
      
      println!("symbol {}, circulating supply/total supply : {}",token.symbol,cs/ts);
        

      }

    
  




pub type Response = Vec<CoinInfo>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    #[serde(rename = "current_price")]
    pub current_price: f64,
    #[serde(rename = "market_cap")]
    pub market_cap: i64,
    #[serde(rename = "market_cap_rank")]
    pub market_cap_rank: i64,
    #[serde(rename = "fully_diluted_valuation")]
    pub fully_diluted_valuation: Option<i64>,
    #[serde(rename = "total_volume")]
    pub total_volume: f64,
    #[serde(rename = "high_24h")]
    pub high_24h: f64,
    #[serde(rename = "low_24h")]
    pub low_24h: f64,
    #[serde(rename = "price_change_24h")]
    pub price_change_24h: f64,
    #[serde(rename = "price_change_percentage_24h")]
    pub price_change_percentage_24h: f64,
    #[serde(rename = "market_cap_change_24h")]
    pub market_cap_change_24h: f64,
    #[serde(rename = "market_cap_change_percentage_24h")]
    pub market_cap_change_percentage_24h: f64,
    #[serde(rename = "circulating_supply")]
    pub circulating_supply: f64,
    #[serde(rename = "total_supply")]
    pub total_supply: Option<f64>,
    #[serde(rename = "max_supply")]
    pub max_supply: Option<f64>,
    pub ath: f64,
    #[serde(rename = "ath_change_percentage")]
    pub ath_change_percentage: f64,
    #[serde(rename = "ath_date")]
    pub ath_date: String,
    pub atl: f64,
    #[serde(rename = "atl_change_percentage")]
    pub atl_change_percentage: f64,
    #[serde(rename = "atl_date")]
    pub atl_date: String,
    pub roi: Option<Roi>,
    #[serde(rename = "last_updated")]
    pub last_updated: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Roi {
    pub times: f64,
    pub currency: String,
    pub percentage: f64,
}
}
