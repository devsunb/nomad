use executor::BackgroundSpawner;

use crate::HttpClient;

/// TODO: docs.
#[derive(Clone)]
pub struct UreqClient<Spawner> {
    agent: ureq::Agent,
    spawner: Spawner,
}

impl<Spawner> UreqClient<Spawner> {
    /// Creates a new `UreqClient` with the given `Spawner`.
    pub fn new(agent: ureq::Agent, spawner: Spawner) -> Self {
        Self { agent, spawner }
    }
}

impl<Spawner: BackgroundSpawner + Sync> HttpClient for UreqClient<Spawner> {
    type Error = ureq::Error;

    fn send(
        &self,
        request: http::Request<String>,
    ) -> impl Future<Output = Result<http::Response<String>, Self::Error>>
    {
        let agent = self.agent.clone();

        self.spawner.spawn(async move {
            let response = agent.run(request)?;
            let (parts, mut body) = response.into_parts();
            let body = body.read_to_string()?;
            Ok(http::Response::from_parts(parts, body))
        })
    }
}
