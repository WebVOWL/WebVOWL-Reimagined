use actix_web::{dev::ServiceRequest, dev::ServiceRespone, Error};
use futures_util::future::LocalBoxFuture;

pub fn cookie_handler<S>(
    req: ServiceRequest,
    srv: S,
) -> LocalBoxFuture<'static, Result<ServiceRespone, Error>>
where
    S: Fn(ServiceRequest) -> LocalBoxFuture<'static, Result<ServiceRespone, Error>> + 'static,
{
    let cookies_accepted = req
        .cookie("accepted")
        .map(|c| c.value() == "true")
        .unwrap_or(false);

    req.extensions_mut().insert(cookies_accepted);

    Box::pin(async move {srv(req).await})
}