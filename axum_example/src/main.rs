use std::collections::HashSet;
use axum::body::Body;
use axum::http::Request;

use axum::middleware::{from_fn, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Extension, Json, Router};
use axum_example::grants_util::AxumGrantsResponse;
use axum_grants::{protect, protect_diy};
use serde_json::json;

pub mod grants_util;

#[derive(Debug, Clone, Default)]
pub struct Claims {
    pub user_id: u64,
    pub user_name: String,
    pub perms: HashSet<String>,
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2000").await.unwrap();
    let router = Router::new().route("/hi", get(hi_handler));
    axum::serve(listener, router.route_layer(from_fn(auth_middle_ware)))
        .await
        .unwrap();
}


///
#[protect_diy(all("opt_crt", "opt_del"))]
async fn hi_handler(Extension(claims): Extension<Claims>) -> impl IntoResponse {
    Json(json!(
        {
            "cd": "0",
            "msg": "success",
            "data": "your business data"
        }
    )).into_response()
}

async fn auth_middle_ware(mut req: Request<Body>, next: Next) -> Response<Body> {
    let uri = req.uri().to_string();
    println!("into middle ware, uri: {} ", uri);

    // your claims can come from a database, Redis, or JWT
    let vec = vec!["opt_crt".to_string(), "opt_del".to_string()];
    let hashset: HashSet<String> = vec.into_iter().collect();
    let claims = Claims {
        user_id: 100,
        user_name: "hui".to_string(),
        perms: hashset,
    };

    // insert the claims into extensions
    req.extensions_mut().insert(claims);

    // do next
    next.run(req).await
}
