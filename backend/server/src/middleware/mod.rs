use salvo::{http::ResBody, writer::Json, Depot, FlowCtrl, Request, Response};
use serde_json::Value;

mod response;
use response::AppResponse;

#[salvo::handler]
pub async fn set_status_code(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    tracing::info!("some middleware");

    ctrl.call_next(req, depot, res).await;

    let body_ref = res.body();
    tracing::info!(
        "Body Shit {} {} {} {}",
        body_ref.is_none(),
        body_ref.is_once(),
        body_ref.is_chunks(),
        body_ref.is_stream()
    );
    // res.write_body("foo").expect("msg");
    // I'd love to set the status code in the body in every endpoint but...
    let status = res.status_code().unwrap_or_default();
    tracing::info!("Res status: {:?}", status);
    // Currently I never need to worry about the other cases.
    if let ResBody::Once(bytes) = res.take_body() {
        // Always valid for my purposes.
        let json: Value = serde_json::from_slice(&bytes).unwrap();
        let response = if res.status_error().is_some() {
            AppResponse::from_error(status.as_u16(), json)
        } else {
            AppResponse::from_response(status.as_u16(), json)
        };
        res.render(Json(response));
    }
    // res.render(Json(AppResponse {
    // status_code: status.as_u16(),
    // details: (),
    // }));
}
