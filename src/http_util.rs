extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use self::hyper::Client;
use self::hyper_tls::HttpsConnector;
use self::tokio_core::reactor::Core;

const HTTP: &str = "http://";
const HTTPS: &str = "https://";

pub fn test() {
    let mut core  = Core::new().unwrap();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

    let res = core.run(client.get("Https://hyper.rs".parse().unwrap())).unwrap();
    assert_eq!(res.status(), self::hyper::Ok);
}

struct Host {
    protocol : String,
    domain : String
}