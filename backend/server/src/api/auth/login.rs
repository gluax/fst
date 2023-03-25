use serde_json::json;

use crate::errors::AppResponse;

use super::*;

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) -> Result<()> {
    Login::validate(req).await?;
    res.set_status_code(StatusCode::OK);
    // TODO: how to clean this up?
    res.render(AppResponse::<_, ()>::from_response(
        200,
        json!({
            "status": "Logged in.",
        }),
    ));
    Ok(())
}
