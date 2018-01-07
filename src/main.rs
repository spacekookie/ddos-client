//! A simple http client which updates ddos with the current IP

extern crate whatsmyip;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use whatsmyip::WhatsMyIp;
use std::fs::{self, File};
use std::io::prelude::*;
use std::{time, thread, env};


#[derive(Debug, Serialize)]
pub struct Signature {
    pub signature: String,
    pub key_id: String,
}

#[derive(Debug, Serialize)]
struct Host {
    auth: Signature,
    name: String,
    ip: String,
}


fn get_authorized(path: &str) -> (String, String) {

        let items = match fs::read_dir(path) {
            Ok(file) => file,
            _ => panic!("'auth' directory not found")
        };

        /* Loop over "secrets" - keys are file names */
        for item in items {
            let path = match item {
                Ok(i) => i.path(),
                _ => continue,
            };

            let name = match path.as_path().file_name() {
                Some(i) => String::from(i.to_str().unwrap()),
                _ => continue,
            };

            let mut secret_f = File::open(&path).unwrap();
            let mut secret = String::new();
            secret_f.read_to_string(&mut secret).unwrap();
            drop(secret_f);

            /* Store name-secret combo in map */
            return (name, secret);
        };

        return ("".to_owned(), "".to_owned());
    }


fn main() {
    let hostname = env::args().nth(1).unwrap();

    let (name, secret) = get_authorized("auth/");

    loop {
        let addrs = WhatsMyIp::new().http_limit(Some(1)).find().unwrap();

        let host = Host {
            auth: Signature {
                signature: secret.clone(),
                key_id: name.clone(),
            },
            name: hostname.clone(),
            ip: format!("{}", addrs[0]),
        };

        let client = reqwest::Client::new();
        let resp = client.post("http://localhost:8000/host/kookiejar.tech").json(&host).send().unwrap().text().unwrap();
        println!("{:?}", resp);

        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);

    }
}
