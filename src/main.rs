use fastly::http::StatusCode;
use fastly::{Error, Request, Response};

#[fastly::main]
fn main(_req: Request) -> Result<Response, Error> {
    // Log service version
    println!("FASTLY_SERVICE_VERSION: {}", std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new()));
    
    Ok(Response::from_status(StatusCode::OK))
}
