// use std::io;
// extern crate docker;
mod docker;

use docker::Docker;

fn main() {
    let endpoint = "http://127.0.0.1:5000";
    let client = Docker::new(endpoint);
    let images = client.list_images();
    // ListImages
}

