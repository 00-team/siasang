use super::*;

#[routes]
#[get("/")]
#[get("/login")]
#[get("/account")]
#[get("/account/info")]
#[get("/account/password")]
#[get("/account/feedback")]
#[get("/account/reviews")]
#[get("/account/saved")]
#[get("/account/suggestions")]
#[get("/rules")]
#[get("/privacy")]
#[get("/result/recruit-link")]
#[get("/result/transaction")]
async fn app_index() -> HttpResponse {
    let body = std::fs::read_to_string("./rah/dist/index.html")
        .unwrap_or_else(|_| "index.html not found".to_string());

    HttpResponse::Ok().content_type(ContentType::html()).body(body)
}
