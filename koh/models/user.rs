use std::pin::Pin;

use actix_web::{
    FromRequest, HttpMessage, HttpRequest, dev::Payload, web::Data,
};

use crate::{AppErr, models::auth::AuthAdmin, state::AdminSessions};

#[derive(Clone, Copy)]
pub struct Admin;

impl FromRequest for Admin {
    type Error = AppErr;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(rq: &HttpRequest, _: &mut Payload) -> Self::Future {
        let rq = rq.clone();

        Box::pin(async move {
            if let Some(admin) = rq.extensions().get::<Admin>() {
                return Ok(*admin);
            }

            let ass = rq.app_data::<Data<AdminSessions>>().unwrap();

            let AuthAdmin { id, token } = AuthAdmin::try_from(&rq)?;
            ass.check_session(id, &token).await?;
            let admin = Admin;

            rq.extensions_mut().insert(admin);
            Ok(admin)
        })
    }
}
