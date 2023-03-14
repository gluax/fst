use super::*;

mod login;
pub use login::*;
// mod register;
// pub use register::*;

#[derive(Debug)]
pub enum Auth {
    Login(login::Login),
    // Register(register::Register),
}

impl Piece for Auth {
    fn render(self, res: &mut Response) {
        tracing::info!("Auth");
        match self {
            Self::Login(l) => l.render(res),
            // Self::Register(r) => r.write(req, depot, res).await,
        };
    }
}
