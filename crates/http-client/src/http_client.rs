use core::error::Error;

/// A trait representing an HTTP client.
pub trait HttpClient: Clone + Send + 'static {
    /// The type of error that can occur after [`send`](HttpClient::send)ing a
    /// request.
    type Error: Error + Send + 'static;

    /// Asynchronously sends an HTTP request and waits for the response.
    fn send(
        &self,
        request: http::Request<String>,
    ) -> impl Future<Output = Result<http::Response<String>, Self::Error>> + Send;
}
