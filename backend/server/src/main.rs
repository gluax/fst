use std::{borrow::Cow, collections::HashMap, sync::Arc};

mod api;
mod errors;
mod middleware;
mod models;
mod utils;

use parking_lot::RwLock;
use salvo::{
    affix,
    jwt_auth::QueryFinder,
    prelude::{JwtAuth, TcpListener},
    Router, Server,
};
use serde::{Deserialize, Serialize};

const SECRET_KEY: &str = "SECRET_KEY";

#[derive(Debug, Clone, Default)]
pub struct Store {
    pub users: Arc<RwLock<HashMap<String, String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims<'a> {
    username: Cow<'a, str>,
    exp: i64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_file(true)
        .init();

    let auth_handler: JwtAuth<JwtClaims> = JwtAuth::new(SECRET_KEY.to_owned())
        .with_finders(vec![
            // Box::new(HeaderFinder::new()),
            Box::new(QueryFinder::new("jwt_token")),
            // Box::new(CookieFinder::new("jwt_token")),
        ])
        .with_response_error(false);

    let router = Router::with_hoop(auth_handler)
        .hoop(affix::inject(Store::default()))
        .hoop(salvo::logging::Logger)
        .hoop(middleware::set_status_code)
        .push(Router::with_path("login").post(crate::api::login));
    tracing::info!("Listening on http://127.0.0.1:8888");
    Server::new(TcpListener::bind("127.0.0.1:8888"))
        .serve(router)
        .await;
}
