use serde_json::json;

use crate::errors::AppResponse;

use super::*;

#[handler]
pub async fn login(data: Login<'_>, res: &mut Response) -> Result<()> {
    // NICE
    data.validate()?;
    res.set_status_code(StatusCode::OK);
    res.render(AppResponse::<_, ()>::from_response(
        200,
        json!({
            "status": "Logged in.",
        }),
    ));
    Ok(())
}
