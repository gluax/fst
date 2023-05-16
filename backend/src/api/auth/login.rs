use super::*;

#[handler]
#[autometrics::autometrics]
pub async fn login(data: Login<'_>, res: &mut Response) -> salvo::Result<()> {
    data.validate()?;

    res.render(salvo::writer::Json(json!({
        "status": "Logged in.",
    })));
    Ok(())
}
