extern crate hyper;

use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use hyper::Client;

fn read_url(client: Client, url: &str, response: &mut String) {
    let mut res = client.get(url).send().unwrap();
    res.read_to_string(response).unwrap();
}

fn read_file(filename: &str, content: &mut String) {
    let path = Path::new(filename);
    let display = path.display();
    let mut file = File::open(&path).unwrap(); /*
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };
    */
    match file.read_to_string(content) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => (),
    }}

fn main() {
    let client = Client::new();
    let mut body = String::new();
    // read_url(client, "http://www.floatrates.com/daily/eur.json", &mut body);
    read_file("cur.json", &mut body);
    println!("{}", body);
}
