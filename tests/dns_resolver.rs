use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    client::HttpConnector,
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server,
};

use http_resolve::BlockLocalhostResolver;

async fn handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World!".into()))
}

async fn spawn_server() {
    tokio::spawn(async {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });
        let server = Server::bind(&addr).serve(make_svc);

        server.await.unwrap();
    });
}

#[tokio::test]
async fn test_localtest() {
    spawn_server().await;
    let connector = HttpConnector::new();
    let request = Request::get("http://localtest.me:8080")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert_eq!(result.is_err(), false);

    let connector = HttpConnector::new_with_resolver(BlockLocalhostResolver::default());
    let request = Request::get("http://localtest.me:8080")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert_eq!(result.is_err(), true);
}

#[tokio::test]
async fn test_localhost() {
    spawn_server().await;
    let connector = HttpConnector::new();
    let request = Request::get("http://localhost:8080")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert_eq!(result.is_err(), false);

    let connector = HttpConnector::new_with_resolver(BlockLocalhostResolver::default());
    let request = Request::get("http://localhost:8080")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert_eq!(result.is_err(), true);
}

#[tokio::test]
async fn test_proper_url() {
    let connector = HttpConnector::new_with_resolver(BlockLocalhostResolver::default());
    let request = Request::get("http://fettblog.eu")
        .body(Body::empty())
        .unwrap();
    let tx = Client::builder().build::<_, Body>(connector);
    let result = tx.request(request).await;
    assert_eq!(result.is_err(), false);
}
