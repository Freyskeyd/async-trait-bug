#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::used_underscore_binding)]

use async_trait::async_trait;
use reqwest::Client as HttpClient;
pub enum ClientError {
    HttpRequest,
}
#[async_trait]
pub trait FetchUserFeed {
    async fn fetch_user_feed(&self, user_id: &str) -> Result<(), ClientError>;
}
pub struct Test {}

#[async_trait]
impl FetchUserFeed for Test {
    async fn fetch_user_feed(&self, _user_id: &str) -> Result<(), ClientError> {
        let _client = HttpClient::new();

        let variables = "";

        let _query = vec![("query_hash", "*"), ("variables", variables)];

        Err(ClientError::HttpRequest)
    }
}
