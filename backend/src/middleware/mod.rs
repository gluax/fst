use salvo::{Depot, FlowCtrl, Request, Response};

#[salvo::handler]
pub async fn set_status_code(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    // tracing::info!("set_status_code");
    ctrl.call_next(req, depot, res).await;
    // let status = res.status_code().unwrap_or_default();
    // Currently I never need to worry about the other cases.
    // if let ResBody::Once(bytes) = res.take_body() {
    //     // Always valid for my purposes.
    //     // TODO: inefficient though... we double serialize now and we deserialize once...
    //     // TODO: just move this logic to proc macros
    //     let json: Value = serde_json::from_slice(&bytes).unwrap();
    //     let response = if res.status_error().is_some() {
    //         AppResponse::from_error(status.as_u16(), json)
    //     } else {
    //         AppResponse::from_response(status.as_u16(), json)
    //     };
    //     res.render(Json(response));
    // }
}
