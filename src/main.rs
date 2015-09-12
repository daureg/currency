extern crate hyper;

use std::io::Read;
use hyper::Client;

fn read_url(client: Client, url: &str, response: &mut String) {
    let mut res = client.get(url).send().unwrap();
    res.read_to_string(response).unwrap();
}

fn main() {
    let client = Client::new();
    let mut body = String::new();
    read_url(client, "http://www.floatrates.com/daily/eur.json", &mut body);
    println!("{}", body);
}
