use crate::HttpClient;

/// TODO: docs.
#[derive(Clone, Default)]
pub struct MockHttpClient {}

impl HttpClient for MockHttpClient {
    type Error = axum_core::Error;

    async fn send(
        &self,
        _request: http::Request<String>,
    ) -> Result<http::Response<String>, Self::Error> {
        todo!();
    }
}
