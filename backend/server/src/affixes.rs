use mongodb::Client;
use salvo::{affix, Router};

pub async fn attach_affixes(router: Router) -> Router {
    // TODO config file
    // TODO move to models? utils?
    let mongodb_uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://127.0.0.1:27017".into());
    // TODO handle this expect call
    let client = Client::with_uri_str(mongodb_uri)
        .await
        .expect("failed to connect");
    let db = client.database("app_db");

    router.hoop(affix::inject(db))
}
