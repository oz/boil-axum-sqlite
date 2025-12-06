use axum::{Router, routing::get, serve::Serve};
use std::error::Error;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use app_state::AppState;
pub mod app_state;
pub mod routes;

// This struct encapsulates our application-related logic.
pub struct Application {
    pub address: String,
    server: Serve<TcpListener, Router, Router>,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .route("/", get(routes::root))
            .fallback_service(ServeDir::new("assets"))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
