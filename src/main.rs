
    

#![allow(unstable)]
extern crate hyper;

use std::os;
use std::old_io::stdout;
use std::old_io::util::copy;

use hyper::Client;

fn main() {
    let mut url = "http://reddit.com/users/SlightlyCyborg/about/.json";

println!("{}",url);


    let mut client = Client::new();

    let mut res = match client.get(url).send() {
        Ok(res) => res,
        Err(err) => panic!("Failed to connect: {:?}", err)
    };

    println!("Response: {}", res.status);
    println!("Headers:\n{}", res.headers);
    match copy(&mut res, &mut stdout()) {
        Ok(..) => (),
        Err(e) => panic!("Stream failure: {:?}", e)
    };

}
