extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;

use self::hyper::Client;
use self::hyper_tls::HttpsConnector;
use self::tokio_core::reactor::Core;
use self::futures::Future;

const HTTPS: String = String::from("https://");

pub fn test() {
    let mut core  = Core::new().unwrap();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

    let res = core.run(client.get("Https://hyper.rs".parse().unwrap())).unwrap();
    assert_eq!(res.status(), self::hyper::Ok);
}

pub fn get(h: &Host, endpoint: &str) -> Result<Future::Item, Future::Error> {
    let mut core = Core::new().unwrap();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());
    core.run(client.get((h.protocol + h.domain + endpoint).parse().unwrap()))
}

pub fn from(protocol: String, domain: String) -> Host {
    Host {
        protocol,
        domain
    }
}

pub struct Host {
    protocol : String,
    domain : String
}