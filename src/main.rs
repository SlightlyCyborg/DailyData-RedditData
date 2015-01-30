extern crate hyper

fn main(){
    println!("This is a simple program that harvests user data from reddit");
    // Create a client.
    let mut client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get("http://www.reddit.com/users/SlightlyCyborg/about/.json")
        // set a header
        .header(Connection(vec![Close]))
        // let 'er go!
        .send();
    // Read the Response.
    let body = res.read_to_string().unwrap();

    println!("Response: {}", res);

}
