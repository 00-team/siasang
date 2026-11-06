use crate::AppErr;
use crate::models::auth::AuthOwj;
use crate::state::SiasangDb;
use actix_web::{
    FromRequest, HttpMessage, HttpRequest, dev::Payload, web::Data,
};
use std::collections::HashSet;
use std::pin::Pin;

#[derive(Clone)]
#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct Owj {
    pub id: u64,
    pub token: String,
    pub scopes: HashSet<String>,
}

impl FromRequest for Owj {
    type Error = AppErr;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(rq: &HttpRequest, _: &mut Payload) -> Self::Future {
        let rq = rq.clone();

        Box::pin(async move {
            if let Some(owj) = rq.extensions().get::<Owj>() {
                return Ok(owj.clone());
            }

            let db = rq.app_data::<Data<SiasangDb>>().unwrap();

            let AuthOwj { id, token } = AuthOwj::try_from(&rq)?;
            let Some(owj) = db.owjes.get(&id) else {
                return crate::err!(BadAuth);
            };

            if owj.token != token {
                return crate::err!(BadAuth);
            }

            rq.extensions_mut().insert(owj.clone());
            Ok(owj.clone())
        })
    }
}
