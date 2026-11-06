use crate::Config;
use crate::docs::UpdatePaths;
use crate::models::Horp;
use crate::state::AdminSessions;
use actix_web::post;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Scope};
use crate::models::Admin;
use actix_web::cookie::{Cookie, SameSite, time::Duration};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags((name = "admin::auth")),
    paths(r_login, r_logout),
    servers((url = "/auth")),
    modifiers(&UpdatePaths)
)]
pub struct ApiDoc;

fn auth_cookie(id: u64, token: &str) -> Cookie<'static> {
    Cookie::build("x-siasang-auth-admin", format!("{id}:{token}"))
        .path("/")
        .secure(!cfg!(debug_assertions))
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(Duration::seconds(Config::TOKEN_LIFE as i64))
        .finish()
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
struct LoginBody {
    password: String,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
struct LoginInfo {
    id: u64,
    token: String,
}

#[utoipa::path(
    post,
    request_body = LoginBody,
    responses((status = 200, body = LoginInfo))
)]
/// Login
#[post("/login/")]
async fn r_login(body: Json<LoginBody>, ass: Data<AdminSessions>) -> Horp {
    let conf = Config::get();

    if !crate::utils::verify_password(&body.password, &conf.master_hash) {
        return crate::err!(BadAuth);
    }

    let (id, token) = ass.new_session().await;

    Ok(HttpResponse::Ok()
        .cookie(auth_cookie(id, &token))
        .json(LoginInfo { id, token }))
}

#[utoipa::path(post, responses((status = 200)))]
#[post("/logout/")]
/// Logout
async fn r_logout(_: Admin) -> HttpResponse {
    let cook = Cookie::build("x-siasang-auth-admin", "heech")
        .path("/")
        .secure(!cfg!(debug_assertions))
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(Duration::seconds(1))
        .finish();

    HttpResponse::Ok().cookie(cook).finish()
}

pub fn router() -> Scope {
    Scope::new("/auth").service(r_login).service(r_logout)
}
