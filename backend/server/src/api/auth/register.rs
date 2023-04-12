use super::*;

#[handler]
#[autometrics::autometrics]
pub async fn register(data: Register<'_>, res: &mut Response) -> salvo::Result<()> {
    data.validate()?;

    let collection = UserCollection::new();
    collection.create(data).await?;

    // TODO what data to show here
    res.render(salvo::writer::Json(json!({
        "status": "Registered.",
    })));
    Ok(())
}
