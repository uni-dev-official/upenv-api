mod auth;
mod error;
mod models;
mod routes;
mod supabase;

use std::{env, sync::Arc};

use axum::Router;
use dotenvy::dotenv;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};

use crate::{routes::router, supabase::SupabaseClient};

#[derive(Clone)]
pub struct AppState {
    pub supabase: Arc<SupabaseClient>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            env::var("RUST_LOG")
                .unwrap_or_else(|_| "upenv_api=debug,tower_http=info".into()),
        )
        .init();

    let supabase_url = env::var("SUPABASE_URL")
        .expect("SUPABASE_URL must be set");
    let supabase_anon_key = env::var("SUPABASE_ANON_KEY")
        .expect("SUPABASE_ANON_KEY must be set");

    let state = AppState {
        supabase: Arc::new(SupabaseClient::new(
            supabase_url,
            supabase_anon_key,
        )),
    };

    let cors = build_cors();

    let app = Router::new()
        .merge(router(state.clone()))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state);

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".into());
    let address = format!("{host}:{port}");

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("failed to bind API listener");

    tracing::info!("UpEnv API listening on http://{address}");

    axum::serve(listener, app)
        .await
        .expect("API server failed");
}

fn build_cors() -> CorsLayer {
    use axum::http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    };

    let origins = env::var("CORS_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:1420".into());

    let allowed_origins = origins
        .split(',')
        .filter_map(|origin| origin.trim().parse::<HeaderValue>().ok())
        .collect::<Vec<_>>();

    CorsLayer::new()
        .allow_origins(allowed_origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_credentials(true)
}
