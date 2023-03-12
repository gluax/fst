use salvo::writer::Json;

use super::*;

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) -> Result<()> {
    tracing::info!("login");
    Login::validate(req).await?;
    // TODO: StatusCode::OK.as_u16();
    res.set_status_code(StatusCode::OK);
    res.render(Json("ok"));
    Ok(())
}
