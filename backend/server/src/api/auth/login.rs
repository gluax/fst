use serde_json::json;

use super::*;

#[handler]
pub async fn login(data: Login<'_>, res: &mut Response) -> salvo::Result<()> {
    data.validate()?;
    res.render(salvo::writer::Json(json!({
        "msg": "Logged in.",
    })));
    Ok(())
}
