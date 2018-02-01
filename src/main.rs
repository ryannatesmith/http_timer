extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

fn main() {
    let mut core  = Core::new().unwrap();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

    let res = core.run(client.get("Https://hyper.rs".parse().unwrap())).unwrap();
    assert_eq!(res.status(), ::hyper::Ok);
}
