//#![feature(custom_derive, plugin)]
//#![plugin(serde_macros)]
//#![plugin(clippy)]

extern crate serde_json;
extern crate hyper;

use serde_json::Value;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use hyper::Client;
use std::collections::BTreeMap;

fn read_url(client: Client, url: &str, response: &mut String) {
    let mut res = client.get(url).send().unwrap();
    res.read_to_string(response).unwrap();
}

fn read_file(filename: &str, content: &mut String) {
    let path = Path::new(filename);
    let display = path.display();
    let mut file = File::open(&path).unwrap();
    match file.read_to_string(content) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           Error::description(&why)),
        Ok(_) => (),
    }
}

#[derive(Debug)]
struct Currency {
    short_name: String,
    full_name: String,
    euro_rate: f64,
}

impl Currency {
    fn new(short_name: &str, full_name: &str, euro_rate: f64) -> Currency {
        Currency {
            short_name: short_name.to_string(),
            full_name: full_name.to_string(),
            euro_rate: euro_rate,
        }
    }
}

fn read_currencies(filename: &str) -> BTreeMap<String, Currency> {
    let mut data = String::new();
    read_file(filename, &mut data);
    let raw_json: Value = serde_json::from_str(&*data).unwrap();
    let json = raw_json.as_object().unwrap();
    let mut res = BTreeMap::new();
    for (key, cur) in json.iter() {
        let cur = cur.as_object().unwrap();
        let name = cur.get("name").unwrap().as_string().unwrap();
        let rate = cur.get("rate").unwrap().as_f64().unwrap();
        res.insert(key.to_string(), Currency::new(key, name, rate));
    }
    return res;
}

fn main() {
    let client = Client::new();
    let mut body = String::new();
    // read_url(client, "http://www.floatrates.com/daily/eur.json", &mut body);
    read_file("cur.json", &mut body);
    let C = read_currencies("cur.json");
    println!("{:?}", C.get("usd").unwrap());
}
