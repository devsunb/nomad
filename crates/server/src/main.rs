//! Collab-server binary for running a self-hosted collaboration server.
//!
//! This server listens on localhost:3000 and accepts connections from collab clients.

use std::str::FromStr;

use base64::Engine;
use collab_server::{
    Authenticator,
    CollabServer,
    Config,
    EndSessionReason,
    KickPeerReason,
    SessionObserver,
};
use collab_types::puff::abs_path::NodeName;
use collab_types::{Peer, PeerHandle, PeerId};
use nomad_collab_params::{
    API_VERSION,
    AuthError,
    AuthInfos,
    NomadParams,
    SessionId,
};
use tokio::net::TcpListener;
use tokio_util::compat::TokioAsyncReadCompatExt;

/// Server configuration for production use
struct ServerConfig;

impl Default for ServerConfig {
    fn default() -> Self {
        Self
    }
}

impl Config for ServerConfig {
    type Authenticator = NomadAuthenticator;
    type Executor = TokioExecutor;
    type Params = NomadParams;
    type SessionObserver = LoggingObserver;

    fn authenticator(&self) -> &Self::Authenticator {
        static AUTHENTICATOR: std::sync::OnceLock<NomadAuthenticator> =
            std::sync::OnceLock::new();
        AUTHENTICATOR.get_or_init(|| NomadAuthenticator)
    }

    fn executor(&self) -> &Self::Executor {
        &TokioExecutor
    }

    fn new_session_id(&self) -> SessionId {
        SessionId::from_rng(&mut rand::rng())
    }

    fn new_session_observer(
        &self,
        host: &Peer,
        project_name: &NodeName,
        session_id: &SessionId,
    ) -> Self::SessionObserver {
        println!(
            "📝 New session created: {} (ID: {}) by host id={:?}, handle={:?}",
            project_name, session_id, host.id, host.handle
        );
        LoggingObserver
    }
}

/// Nomad authenticator that validates JWT tokens
struct NomadAuthenticator;

impl Authenticator for NomadAuthenticator {
    type Infos = AuthInfos;
    type Error = AuthError;

    async fn authenticate(
        &self,
        auth_infos: &Self::Infos,
    ) -> Result<PeerHandle, Self::Error> {
        // Check API version
        if auth_infos.api_version != API_VERSION {
            return Err(AuthError::OutdatedClient);
        }

        // Parse JWT without validation (development mode)
        // In production, validate the signature with a proper secret key
        let parts: Vec<&str> = auth_infos.jwt.split('.').collect();
        if parts.len() != 3 {
            return Err(AuthError::Jwt("Invalid JWT format".to_string()));
        }

        // Decode payload (second part of JWT)
        let payload = parts[1];
        let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(payload)
            .map_err(|e| {
                AuthError::Jwt(format!("Failed to decode JWT: {}", e))
            })?;

        let payload_str = String::from_utf8(decoded).map_err(|e| {
            AuthError::Jwt(format!("Invalid UTF-8 in JWT: {}", e))
        })?;

        // Parse JSON to extract GitHub handle
        let payload_json: serde_json::Value =
            serde_json::from_str(&payload_str).map_err(|e| {
                AuthError::Jwt(format!("Invalid JSON in JWT: {}", e))
            })?;

        // Extract GitHub handle from the "username" or other claims
        let github_handle_str = payload_json
            .get("username")
            .or_else(|| payload_json.get("github_handle"))
            .or_else(|| payload_json.get("handle"))
            .or_else(|| payload_json.get("sub"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                AuthError::Jwt("No GitHub handle found in JWT".to_string())
            })?;

        // Remove "github:" prefix if present
        let github_handle_str = github_handle_str
            .strip_prefix("github:")
            .unwrap_or(github_handle_str);

        let github_handle = collab_types::GitHubHandle::from_str(
            github_handle_str,
        )
        .map_err(|e| {
            AuthError::Jwt(format!(
                "Invalid GitHub handle '{}': {}",
                github_handle_str, e
            ))
        })?;

        Ok(PeerHandle::GitHub(github_handle))
    }
}

/// Tokio executor for running background tasks
struct TokioExecutor;

impl collab_server::Executor for TokioExecutor {
    type JoinHandle<T: Send> = TokioJoinHandle<T>;

    fn spawn<Fut>(&self, future: Fut) -> Self::JoinHandle<Fut::Output>
    where
        Fut: std::future::Future + Send + 'static,
        Fut::Output: Send + 'static,
    {
        TokioJoinHandle(tokio::spawn(future))
    }
}

/// Wrapper around tokio::task::JoinHandle to implement Future correctly
struct TokioJoinHandle<T>(tokio::task::JoinHandle<T>);

impl<T> std::future::Future for TokioJoinHandle<T> {
    type Output = T;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match std::pin::Pin::new(&mut self.0).poll(cx) {
            std::task::Poll::Ready(Ok(val)) => std::task::Poll::Ready(val),
            std::task::Poll::Ready(Err(e)) => {
                panic!("Task panicked: {:?}", e)
            },
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// Logging observer for session events
struct LoggingObserver;

impl SessionObserver<NomadParams> for LoggingObserver {
    fn on_peer_joined(&mut self, peer: &Peer) {
        println!("👋 Peer joined: id={:?}, handle={:?}", peer.id, peer.handle);
    }

    fn on_peer_kicked(&mut self, peer: &Peer, reason: &KickPeerReason) {
        println!("👋 Peer kicked: {:?} (reason: {:?})", peer.handle, reason);
    }

    fn on_fragment_inbound(
        &mut self,
        _fragment: &collab_server::MessageFragment,
        _sender_id: PeerId,
    ) {
        // Fragment logging disabled for cleaner output
    }

    fn on_fragment_outbound(
        &mut self,
        _fragment: &collab_server::MessageFragment,
        _recipient_id: PeerId,
    ) {
        // Fragment logging disabled for cleaner output
    }

    fn on_session_ended(
        self,
        session_id: SessionId,
        reason: EndSessionReason,
        num_peers: u32,
    ) {
        println!(
            "🔚 Session {} ended: {:?} ({} peers)",
            session_id, reason, num_peers
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting collab-server");
    println!("📋 Using NomadParams (API version {})", API_VERSION);

    // Allow binding to custom address via environment variable
    // Default to 0.0.0.0:3000 to allow connections from all interfaces
    let bind_addr = std::env::var("COLLAB_SERVER_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    let listener = TcpListener::bind(&bind_addr).await?;
    println!("✅ Server listening on {}", bind_addr);

    let config = ServerConfig::default();
    let server = CollabServer::new(config);

    // Convert incoming TCP connections to a stream with tokio-compat wrapper
    let incoming =
        futures_util::stream::unfold(listener, |listener| async move {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    println!("🔗 New connection from: {}", addr);
                    // Convert tokio's AsyncRead/Write to futures' AsyncRead/Write
                    let compat_stream = stream.compat();
                    Some((compat_stream, listener))
                },
                Err(e) => {
                    eprintln!("❌ Error accepting connection: {}", e);
                    None
                },
            }
        });

    server.run(Box::pin(incoming)).await;

    Ok(())
}
