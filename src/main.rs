extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Core;
use futures::{Future, Stream};
use futures::future;

use hyper::{Uri, Method, Error};
use hyper::client::{Client, Request};
use hyper::header::{Authorization, Accept, UserAgent, qitem};
use hyper::mime::Mime;
use hyper_tls::HttpsConnector;

use std::io::{self, Read, Write};

use std::str::FromStr;

fn main() {
    let url = Uri::from_str("https://api.github.com/users/mathieu-pousse/orgs").unwrap();

    let mut req = Request::new(Method::Get, url);

    let mut event_loop = Core::new().unwrap();
    let handle = event_loop.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle))
        .build(&handle);

    req.headers_mut().set(UserAgent::new("my-rust"));

    let work = client.request(req).and_then(|result| {

        let mut body = String::new();
        result
            .body()
            .for_each(move |chunk| io::stdout().write_all(&chunk).map(|_| ()))
    });
    let body = event_loop.run(work).unwrap();
    
}
