use super::*;

mod login;
pub use login::*;
// mod register;
// pub use register::*;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Auth {
    Login(login::Login),
    // Register(register::Register),
}

#[salvo::async_trait]
impl Writer for Auth {
    async fn write(mut self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        tracing::info!("Auth");
        match self {
            Self::Login(l) => l.write(req, depot, res).await,
            // Self::Register(r) => r.write(req, depot, res).await,
        };
    }
}
