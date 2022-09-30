use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::{Request, Response, RouteContext};

#[derive(Deserialize, Serialize)]
struct UserConfiguration {
    name: String,
}

pub fn ping(_: Request, _: RouteContext<()>) -> Result<Response, worker::Error> {
    Response::from_json(&json!({ "server": "ok" }))
}

/// Return current user configuration stored in Cloudflare's KV
pub async fn get_user_conf(_: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    let kv = ctx.kv("KV_LIFE_RULES")?;
    let user_id = ctx.param("user_id");
    match user_id {
        Some(user_id) => {
            let user_info = kv.get(user_id).json::<UserConfiguration>().await?;
            if let Some(user_info) = user_info {
                Response::from_json(&json!(user_info))
            } else {
                Response::from_json(&json!({"msg": "User not found!"}))
            }
        }
        _ => Response::error("Missing user id", 400),
    }
}

/// Return current worker version.
pub fn get_worker_version(_: Request, ctx: RouteContext<()>) -> Result<Response, worker::Error> {
    let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
    Response::ok(version)
}
