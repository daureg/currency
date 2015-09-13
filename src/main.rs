//#![feature(custom_derive, plugin)]
//#![plugin(serde_macros, clippy)]

extern crate chrono;
extern crate hyper;
extern crate serde_json;
extern crate term_grid;
extern crate ansi_term;


use ansi_term::Colour::{Yellow};
use chrono::datetime::*;
use hyper::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use term_grid::{Grid, GridOptions, Direction, Filling};

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
    update: DateTime<chrono::offset::fixed::FixedOffset>,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.full_name, Yellow.paint(&self.short_name))
    }
}

impl Currency {
    fn new(short_name: &str, full_name: &str, euro_rate: f64,
           update: DateTime<chrono::offset::fixed::FixedOffset>) -> Currency {
        Currency {
            short_name: short_name.to_string(),
            full_name: full_name.to_string(),
            euro_rate: euro_rate,
            update: update.clone(),
        }
    }
}

fn read_currencies(filename: &str) -> HashMap<String, Currency> {
    let mut data = String::new();
    read_file(filename, &mut data);
    let raw_json: Value = serde_json::from_str(&*data).unwrap();
    let json = raw_json.as_object().unwrap();
    let mut res = HashMap::new();
    for (key, cur) in json.iter() {
        let cur = cur.as_object().unwrap();
        let name = cur.get("name").unwrap().as_string().unwrap();
        let rate = cur.get("rate").unwrap().as_f64().unwrap();
        let str_date = cur.get("date").unwrap().as_string().unwrap();
        let update = DateTime::parse_from_rfc2822(str_date).unwrap();
        res.insert(key.to_string(), Currency::new(key, name, rate, update));
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
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(5),
        direction: Direction::TopToBottom,
    });
    let mut actual: Vec<_> = C.values().map(|ref x| format!("{}", x)).collect();
    actual.sort();
    for name in &actual {
        grid.add((*name).clone().into());
    }
    println!("{}", grid.fit_into_width(80).unwrap());
}
