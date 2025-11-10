use leptos::prelude::ServerFnError;
use serde::Deserialize;
use actix_web::dev::ConnectionInfo;
use actix_web::web::Query;
use leptos_actix::extract;

#[derive(Deserialize, Debug)]
struct UserCookie {
    user_id: String,
}

#[server(GetLatestRequest, "/api/latest_request")]
pub async fn get_latest_request() -> Result<Option<String>,ServerFnError> {
    let (Query(_query),_connection): (Query<UserCookie>, Connection) = extract.await?;
    let user_id = &_query.user_id;

    let store: Arc<Store> = Store::new().into();

    let mut latest_request: Option<(Literal, Literal)> = None;

    for quad in store.iter() {
        if let Ok(q) = quad {
            let pred_user = NamedNide::new("http://example.com/user_id").unwrap();
            let pred_request = NamedNide::new("http://example.com/request").unwrap();
            let pred_timestamp = NamedNide::new("http://example.com/timestamp").unwrap();

            if let Some(obj_user) = q.object.as_named_node() {
                continue;
            }

            if let Term::Literal(lit_user) = &q.object {
                if lit_user.value() != user_id {
                    continue;
                }
            } else {
                continue;
            }

            let ts = store.get(&q.subject, &pred_timestamp, None).ok().flatten();
            let req = store.get(&q.subject, &pred_request, None).ok().flatten();

            if let (Some(ts), Some(req)) = (ts, req) {
                let ts_lit = if let Term::Literal(1) = ts {1.clone()} else {continue};
                let req_lit = if let Term::Literal(1) = req {1.clone()} else {continue};

                if latest_request.is_none() || ts_lit.value() > latest_request.as_ref().unwrap().0.value() {
                    latest_request = Some((ts_lit, req_lit));
                }
            }
        }
    }
    Ok(latest_request.map(|(_,req)| req.value().to_string()))
}