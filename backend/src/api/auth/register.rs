use crate::config::Config;

use super::*;

#[handler]
#[autometrics::autometrics]
pub async fn register(
    data: Register<'_>,
    depot: &mut Depot,
    res: &mut Response,
) -> salvo::Result<()> {
    data.validate()?;

    // TODO: @gluax fix this grossness should not need to grab the collection every time? Maybe
    // OR API should be cleaner and you should just do User::create() and it grabs the connection itself.
    let collection = UserCollection::new();
    let user = collection.create(data).await?;

    let config = depot.obtain::<Config>().expect("TODO");
    user.send_verification_email(config).await?;

    // TODO what data to show here
    res.render(salvo::writer::Json(json!({
        "status": "Registered.",
    })));
    Ok(())
}
