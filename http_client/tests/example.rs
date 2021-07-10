use http_client::{HttpClient, HttpRequest};
use hyper::Method;

#[tokio::test]
async fn reqwest_example() {
    let client = reqwest::Client::new();
    let method = Method::GET;
    let url = "https://www.shimo.im".parse().unwrap();
    let request = HttpRequest::new(method, url, None, None);
    let ret = client.dispatch(request).await.unwrap();
    assert!(ret.status.is_success());
}
