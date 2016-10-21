// #![crate_type = "lib"]
// #![crate_name = "docker"]

extern crate hyper;

use std::io::Read;

pub struct Docker {
    endpoint: &'static str
}

impl Docker {
    pub fn new(endpoint : &'static str) -> Docker {
        Docker { endpoint: endpoint }
    }

    pub fn list_images(&self) {
        let api: &str = "/images/json";
        let url = format!("{}{}", self.endpoint, api);
        docker_list_images(&url)
    }
}

fn docker_list_images(url : &str) {
    let client = hyper::Client::new();
    println!("{}", url);
    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(_) => panic!("Whoops."),
    };
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("I give up."),
    };
    println!("{}", buf);
}

