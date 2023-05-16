// use mongodb::Client;
use salvo::{affix, Router};

use crate::config::Config;

pub fn attach_affixes(router: Router) -> Router {
    let config = Config::from_env().expect("TODO");
    router.hoop(affix::inject(config))
}
