// #![crate_type = "lib"]
// #![crate_name = "docker"]

extern crate hyper;
use std::io::Read;

pub fn new_client(end : &str) {
    let client = hyper::Client::new();
    let url = "http://127.0.0.1:5000/api";
    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(_) => panic!("Whoops."),
    };
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("I give up."),
    };
    println!("buf: {}", buf);
}

