use salvo::writer::Json;
use serde_json::json;

use super::*;

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) -> Result<()> {
    Login::validate(req).await?;
    res.set_status_code(StatusCode::OK);
    res.render(Json(json!({
        "status": "Logged in.",
    })));
    Ok(())
}
