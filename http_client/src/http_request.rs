use hyper::{Body, HeaderMap, Method, Uri};
use url::Url;

pub type Params = std::collections::BTreeMap<String, Option<String>>;

#[derive(Debug)]
pub struct HttpRequest {
    url: Url,
    method: Method,
    body: Body,
    headers: HeaderMap,
}

impl HttpRequest {
    pub fn new<B, H>(method: Method, url: Url, body: B, headers: H) -> Self
    where
        B: Into<Option<Body>>,
        H: Into<Option<HeaderMap>>,
    {
        let body = body.into().unwrap_or(Body::default());
        let headers = headers.into().unwrap_or_default();
        Self {
            url,
            method,
            body,
            headers,
        }
    }
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }
}

impl From<HttpRequest> for hyper::Request<Body> {
    fn from(rqst: HttpRequest) -> Self {
        let HttpRequest {
            url,
            method,
            body,
            headers,
            // params,
        } = rqst;
        // let mut query = url.query_pairs_mut();
        // for (name, value) in params.iter() {
        //     if let Some(value) = value {
        //         query.append_pair(name, value);
        //     } else {
        //         query.append_key_only(name);
        //     }
        // }
        // drop(query);
        let uri: Uri = url.as_str().parse().expect("Invalid Url");
        let mut rqst = hyper::Request::builder()
            .method(method)
            .uri(uri)
            .body(body)
            .expect("Invalid parts");
        *rqst.headers_mut() = headers;
        rqst
    }
}
