#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct CurrencyOverview {
    lines: Vec<Currency>,
    currencyDetails: Vec<ItemInfo>,
    language: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Currency {
    currency_type_name: String,
    pay: Pay,
    //    receive: Receive,
    //    paySparkLine: Paysparkline,
    //    receieveSparkLine: Receivesparkline,
    chaos_equivalent: f64,
    //    lowConfidencePaySparkLine: Lowconfidencepaysparkline,
    //    lowConfidenceReceiveSparkLine: Lowconfidencereceivesparkline,
    details_id: String,
}

impl Currency {
    fn new(name: &str, value: &str) -> Currency {
        Currency {
            currency_type_name: name.to_string(),
            pay: Pay::new(),
            chaos_equivalent: value.parse().unwrap(),
            details_id: Currency::gen_detailsid(name),
        }
    }

    fn gen_detailsid(name: &str) -> String {
        let name = str::replace(name, " ", "-");
        name.to_ascii_lowercase()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ItemInfo {}

#[derive(Debug, Deserialize, Serialize)]
struct Pay {
    id: u32,
    league_id: u32,
    pay_currency_id: u32,
    get_currency_id: u32,
    //    sample_time_utc: TimeformatOfSomeKind,
    count: u32,
    value: f64,
    data_point_count: u32,
    includes_secondary: bool,
    listing_count: u32,
}

impl Pay {
    fn new() -> Pay {
        Pay {
            id: 1,
            league_id: 1,
            pay_currency_id: 1,
            get_currency_id: 1,
            count: 1,
            value: 1.0,
            data_point_count: 1,
            includes_secondary: false,
            listing_count: 1,
        }
    }
}

fn main() {
    let m = Currency::new("Mirror of Kalandra", "15000");
    //    let hi = Currency {
    //        currency_type_name: String::from("Mirror of Kalandra"),
    //        chaos_equivalent: 14542.26,
    //        details_id: String::from("mirror-of-kalandra"),
    //    };
    //
    println!("{}", m.details_id);
    println!("{}", m.chaos_equivalent);
    println!("{}", m.currency_type_name);

    let k = Currency::new("MirrorShard", "2000");

    println!("{}", k.details_id);

    //    let json = r#"
    //        {
    //        "currency_type_name": "Chaos Orb",
    //        "chaos_equivalent": 1.0,
    //        "details_id": "chaos-orb"
    //        }
    //    "#;
    //
    let json = serde_json::to_string(&m).expect("Failed to serialize Currency");
    println!("{}", json);
}
