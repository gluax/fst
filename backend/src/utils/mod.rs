use salvo::Request;

use crate::errors::Result;

pub trait ErrorIf: Default {
    fn error_or_ok(self) -> Result<()>;
}

#[salvo::async_trait]
pub trait Validate {
    type Error: ErrorIf;
    async fn validate(req: &mut Request) -> Result<()>
    where
        Self: 'async_trait;
}

pub trait RoutingGroup {
    // fn group(self) ->
}
