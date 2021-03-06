use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    client::HttpConnector,
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server,
};

use hyper_tls::HttpsConnector;

use http_resolve::BlockLocalhostResolver;

async fn handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World!".into()))
}

async fn spawn_server(port: u16) {
    tokio::spawn(async move {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });
        let server = Server::bind(&addr).serve(make_svc);

        server.await.unwrap();
    });
}

#[tokio::test]
async fn test_localtest() {
    spawn_server(8080).await;
    let connector = HttpConnector::new();
    let request = Request::get("http://localtest.me:8080")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert!(result.is_ok());

    let connector = HttpConnector::new_with_resolver(BlockLocalhostResolver::default());
    let request = Request::get("http://localtest.me:8080")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_localhost() {
    spawn_server(8081).await;
    let connector = HttpConnector::new();
    let request = Request::get("http://localhost:8081")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert!(result.is_ok());

    let connector = HttpConnector::new_with_resolver(BlockLocalhostResolver::default());
    let request = Request::get("http://localhost:8080")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_proper_url() {
    let connector = HttpConnector::new_with_resolver(BlockLocalhostResolver::default());
    let request = Request::get("http://fettblog.eu")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_proper_url_https() {
    let mut connector = HttpConnector::new_with_resolver(BlockLocalhostResolver::default());
    connector.enforce_http(false);
    let connector = HttpsConnector::new_with_connector(connector);
    let request = Request::get("https://fettblog.eu")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_localtest_https_connector() {
    spawn_server(8080).await;
    let mut connector = HttpConnector::new();
    connector.enforce_http(false);
    let connector = HttpsConnector::new_with_connector(connector);

    let request = Request::get("http://localtest.me:8080")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert!(result.is_ok());

    let mut connector = HttpConnector::new_with_resolver(BlockLocalhostResolver::default());
    connector.enforce_http(false);
    let connector = HttpsConnector::new_with_connector(connector);
    let request = Request::get("http://localtest.me:8080")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert!(result.is_err());
}
