use salvo::Listener;
use std::borrow::Cow;

mod api;
mod config;
mod errors;
mod middleware;
mod models;
mod utils;

use salvo::{
    jwt_auth::QueryFinder,
    prelude::{JwtAuth, TcpListener},
    Router, Server,
};
use serde::{Deserialize, Serialize};
// use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter};

const SECRET_KEY: &str = "SECRET_KEY";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims<'a> {
    username: Cow<'a, str>,
    exp: i64,
}

#[tokio::main]
async fn main() {
    let _exporter = autometrics::global_metrics_exporter();
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_file(true)
        .init();
    // let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // let subscriber = tracing_subscriber::registry().with(filter);
    // tracing::subscriber::set_global_default(subscriber).unwrap();

    let config = config::Config::from_env().expect("TODO");

    models::init_db(&config.mongodb_uri).await;

    let auth_handler: JwtAuth<JwtClaims> = JwtAuth::new(SECRET_KEY.to_owned())
        .finders(vec![
            // Box::new(HeaderFinder::new()),
            Box::new(QueryFinder::new("jwt_token")),
            // Box::new(CookieFinder::new("jwt_token")),
        ])
        .response_error(false);

    let router = Router::with_hoop(auth_handler)
        .hoop(salvo::logging::Logger::new())
        .hoop(salvo::affix::inject(config))
        // .hoop(middleware::set_status_code)
        .push(Router::with_path("metrics").get(crate::api::metrics))
        .push(
            Router::with_path("api")
                .push(Router::with_path("login").post(crate::api::login))
                .push(Router::with_path("register").post(crate::api::register)),
        );
    tracing::info!("Listening on http://127.0.0.1:8888");
    let acceptor = TcpListener::new("127.0.0.1:8888").bind().await;
    Server::new(acceptor).serve(router).await;
}
