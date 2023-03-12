use salvo::Response;

#[salvo::handler]
pub async fn headers(_: &mut Response) {
    tracing::info!("headers middleware");
}
