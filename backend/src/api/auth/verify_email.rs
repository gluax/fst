use super::*;

#[handler]
#[autometrics::autometrics]
pub async fn verify_email(data: Register<'_>, res: &mut Response) -> salvo::Result<()> {
    data.validate()?;

    // TODO: @gluax fix this grossness should not need to grab the collection every time? Maybe
    // OR API should be cleaner and you should just do User::create() and it grabs the connection itself.
    let collection = UserCollection::new();

    // TODO what data to show here
    res.render(salvo::writer::Json(json!({
        "status": "Verified.",
    })));
    Ok(())
}
