use std::future::Future;

use reqwest::{self, Response};

pub fn fetch (url: &str, session_cookie: &str) -> impl Future<Output = Result<Response, reqwest::Error>> {
    
    let client = reqwest::Client::new();

    let future = client
        .get(url)
        .header(reqwest::header::COOKIE, format!("session={}", session_cookie))
        .send();
    future

}