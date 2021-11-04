use rest_client::Client;
pub mod reference;
pub mod stocks;

pub use reference::*;
pub use stocks::*;

pub fn client(token: &str) -> Client {
    Client::new("https://api.polygon.io").query_auth(vec![("apiKey", token)])
}

pub fn client_with_url<'a>(url: &'a str, token: &'a str) -> Client<'a> {
    Client::new(url).query_auth(vec![("apiKey", token)])
}
