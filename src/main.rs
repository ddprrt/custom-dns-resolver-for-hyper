mod block_localhost_resolver;
mod has_localhost;

use hyper::client::connect::HttpConnector;
use hyper::http::Request;
use hyper_tls::{HttpsConnector};

use crate::block_localhost_resolver::BlockLocalhostResolver;

use hyper::{Body, Client};

#[tokio::main]
async fn main() {
    let connector = HttpConnector::new_with_resolver(BlockLocalhostResolver::default());

    println!(">>> Accessing localtest.me at port 8080. Make sure your local server runs");
    let request = Request::get("http://localtest.me:8080")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    match tx.request(request).await {
        Ok(res) => println!("{:?}", res),
        Err(err) => println!("Seems like you are calling localhost {:?}", err),
    }

    println!(">>> Accessing fettblog.eu");
    let request = Request::get("http://fettblog.eu")
        .body(Body::empty())
        .unwrap();
    match tx.request(request).await {
        Ok(res) => println!("Allowed to access {:?}", res),
        Err(err) => println!("Seems like you are calling localhost {:?}", err),
    }

    println!(">>> Accessing localhost");
    let request = Request::get("http://localhost")
        .body(Body::empty())
        .unwrap();
    match tx.request(request).await {
        Ok(res) => println!("Allowed to access {:?}", res),
        Err(err) => println!("Seems like you are calling localhost {:?}", err),
    }

    let mut connector = HttpConnector::new_with_resolver(BlockLocalhostResolver::default());
    connector.enforce_http(false);
    let connector = HttpsConnector::new_with_connector(connector);

    println!("");
    println!("------ HTTPS -----");
    println!("");
    
    let tx = Client::builder().build::<_, Body>(connector);
    match tx.get("https://fettblog.eu".parse().unwrap()).await {
        Ok(res) => println!("Allowed to access {:?}", res),
        Err(err) => println!("Seems like you are calling localhost {:?}", err),
    }
}
