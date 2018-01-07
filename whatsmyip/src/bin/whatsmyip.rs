
extern crate whatsmyip;
extern crate env_logger;

use whatsmyip::whatsmyip;

fn main() {

    env_logger::init().unwrap();
    let addr = whatsmyip();
    println!("Address: {:?}", addr);

    // let addrs = WhatsMyIp::new()
    //                 .http_limit(Some(1))
    //                 .find().unwrap();
}
