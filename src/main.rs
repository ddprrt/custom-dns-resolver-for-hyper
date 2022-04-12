mod has_localhost;
mod my_resolver;

use hyper::client::HttpConnector;
use hyper::http::Request;

use crate::my_resolver::MyResolver;

use hyper::{Body, Client};

#[tokio::main]
async fn main() {
    let connector = HttpConnector::new_with_resolver(MyResolver::default());

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
}

#[tokio::test]
async fn test_localtest() {
    let connector = HttpConnector::new_with_resolver(MyResolver::default());
    let request = Request::get("http://localtest.me:8080")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert_eq!(result.is_err(), true);
}

#[tokio::test]
async fn test_localhost() {
    let connector = HttpConnector::new_with_resolver(MyResolver::default());
    let request = Request::get("http://localhost:8080")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert_eq!(result.is_err(), true);
}

#[tokio::test]
async fn test_proper_url() {
    let connector = HttpConnector::new_with_resolver(MyResolver::default());
    let request = Request::get("http://fettblog.eu")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert_eq!(result.is_err(), false);
}
