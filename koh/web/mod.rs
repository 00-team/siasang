use actix_web::dev::HttpServiceFactory;
use actix_web::http::header::ContentType;
use actix_web::middleware::NormalizePath;
use actix_web::{HttpResponse, Scope, routes};

mod app;

// #[derive(Debug, Default)]
// pub struct SsrCache {
//     pub main: RwLock<HashMap<String, Vec<u8>>>,
// }

// #[routes]
// #[get("/admin")]
// #[get("/admin/{path:.*}")]
// async fn admin_index(rq: HttpRequest) -> HttpResponse {
//     let Ok(admin) = Admin::extract(&rq).await else {
//         return not_found().await;
//     };
//     if !admin.perms.perm_any() {
//         return not_found().await;
//     }
//
//     let result = std::fs::read_to_string("../web-admin/dist/index.html")
//         .unwrap_or("err reading admin index.html".to_string());
//     HttpResponse::Ok().content_type(ContentType::html()).body(result)
// }

pub fn router() -> impl HttpServiceFactory {
    Scope::new("").wrap(NormalizePath::trim()).service(app::app_index)
    // .service(admin_index)
}
