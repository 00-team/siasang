use crate::docs::UpdatePaths;
use crate::models::Admin;
use crate::models::Jorp;
use crate::state::SiasangDb;
use actix_web::Scope;
use actix_web::get;
use actix_web::web::{Data, Json};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags((name = "admin::state")),
    paths(r_get),
    servers((url = "/state")),
    modifiers(&UpdatePaths)
)]
pub struct ApiDoc;

#[utoipa::path(
    get,
    responses((status = 200, body = SiasangDb))
)]
/// Get
#[get("/")]
async fn r_get(_: Admin, db: Data<SiasangDb>) -> Jorp<Data<SiasangDb>> {
    Ok(Json(db))
}

pub fn router() -> Scope {
    Scope::new("/state").service(r_get)
}
