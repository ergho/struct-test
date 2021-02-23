//use std;
use serde::{Deserialize, Serialize};
use serde_json::Number;
use serde_json::Result;
use chrono::{DateTime, Utc};
//extern crate reqwest;
//extern crate tokio;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct CurrencyOverview {
    lines: Vec<Currency>,
    //currency_details: Vec<ItemInfo>,
    language: Languages,
}

impl CurrencyOverview {
    pub fn new() -> CurrencyOverview {
        CurrencyOverview {
            lines: Vec::new(),
            language: Languages {
                name: "en".to_string(),
            }
        }
    }
    pub fn add(&mut self, currency: Currency) {
        self.lines.push(currency);
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ItemOverview {
    lines: Vec<Item>,
    language: Languages,
}

// A few more option values than i would have liked...
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Item {
    id: u32,
    name: String,
    icon: String,
    map_tier: u32,
    level_required: u32,
    base_type: Option<String>,
    stack_size: u32,
    variant: Option<String>,
    prophecy_text: Option<String>,
    art_filename: Option<String>,
    links: u32,
    item_class: u32,
    sparkline: Sparkline,
    low_confidence_sparkline: Sparkline,
    implicit_modifiers: Option<Vec<Modifiers>>,
    explicit_modifiers: Option<Vec<Modifiers>>,
    flavour_text: String,
    corrupted: bool,
    gem_level: u32,
    gem_quality: u32,
    item_type: String,
    chaos_value: f64,
    exalted_value: f64,
    count: u32,
    details_id: String,
    trade_info: Option<String>,
    map_region: Option<String>,
    listing_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Modifiers {
    text: String,
    optional: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Currency {
    currency_type_name: String,
    pay: Pay,
    //receive: Option<Receive>,
    //pay_spark_line: Option<Sparkline>,
    //receive_spark_line: Option<Sparkline>,
    chaos_equivalent: f64,
    //low_confidence_pay_spark_line: Option<Sparkline>,
    //low_confidence_receive_spark_line: Option<Sparkline>,
    details_id: String,
}

impl Currency {
    pub fn new(name: &str, value: &str) -> Currency {
        Currency {
            currency_type_name: name.to_string(),
            pay: Pay::new(value),
            //receive,
            //pay_spark_line,
            //receive_spark_line,
            chaos_equivalent: value.parse().unwrap(),
            //low_confidence_pay_spark_line,
            //low_confidence_receive_spark_line,
            details_id: Currency::gen_detailsid(name),
        }
    }

    fn gen_detailsid(name: &str) -> String {
        let name = str::replace(name, " ", "-");
        name.to_ascii_lowercase()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ItemInfo {
    id: u32,
    icon: String,
    name: String,
    trade_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Receive {
    id: u32,
    league_id: u32,
    pay_currency_id: u32,
    get_currency_id: u32,
    sample_time_utc: DateTime<Utc>,
    count: u32,
    value: f64,
    data_point_count: u32,
    includes_secondary: bool,
    listing_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Languages {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Sparkline {
    data: Vec<Option<Number>>,
    total_change: Number,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
struct Pay {
    id: u32,
    league_id: u32,
    pay_currency_id: u32,
    get_currency_id: u32,
    //sample_time_utc: DateTime<Utc>,
    count: u32,
    value: f64,
    data_point_count: u32,
    includes_secondary: bool,
    listing_count: u32,
}

impl Pay {
    fn new(value: &str) -> Pay {
        Pay {
            id: 1,
            league_id: 1,
            pay_currency_id: 1,
            get_currency_id: 1,
            count: 1,
            value: value.parse().unwrap(),
            data_point_count: 1,
            includes_secondary: false,
            listing_count: 1,
        }
    }
}


fn printthing(container: CurrencyOverview) -> Result<String> {
    let k = serde_json::to_string(&container)?;
    //println!("{}", k);
    Ok(k)
}

fn main () {
    let mirror: Currency = Currency::new("Mirror of Kalandra", "2");
    println!("{:?}", mirror);
    let mut container: CurrencyOverview = CurrencyOverview::new();
    container.add(mirror);
    container.add(Currency::new("Exalted Orb", "5"));
    println!("{:?}", container);
    let a = printthing(container);
    println!("{:?}", a);


}
//#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let body = reqwest::get(
//        "https://poe.ninja/api/data/currencyoverview?league=Ritual&type=Currency&language=en",
//    )
//    .await?
//    .text()
//    .await?;
//
//    let item_body = reqwest::get(
//        "https://poe.ninja/api/data/ItemOverview?league=Ritual&type=Invitation&language=en",
//    )
//    .await?
//    .text()
//    .await?;
//
//    let v: CurrencyOverview = serde_json::from_str(&body)?;
//    let item: ItemOverview = serde_json::from_str(&item_body)?;
//
//
//
//    println!("{:?}", v.lines[0].currency_type_name);
//    println!("{:?}", v.lines[0].pay.unwrap());
//    println!("{:?}", v.currency_details[0].name);
//    println!("{:?}", v.language.name);
//    println!("{:?}", item.lines[0].low_confidence_sparkline);
//
//    //maybe do something more than just deserialize now?
//
//    Ok(())
//}
