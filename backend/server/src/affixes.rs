use mongodb::Client;
use salvo::{affix, Router};

pub async fn attach_affixes(router: Router) -> Router {
    // router.hoop(affix::inject(db))
    router
}
