use actix_web::{FromRequest, HttpRequest, dev::Payload};
use actix_web::{HttpResponse, web::Json};
use std::{convert::Infallible, pin::Pin};

use crate::AppErr;

pub type Horp = Result<HttpResponse, crate::AppErr>;
pub type Jorp<T> = Result<Json<T>, crate::AppErr>;

/// Seed Host from connection info with https://
pub struct Sost(String);
impl std::ops::Deref for Sost {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Sost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::ops::Add<&str> for &Sost {
    type Output = String;

    fn add(self, rhs: &str) -> Self::Output {
        self.0.clone() + rhs
    }
}

impl FromRequest for Sost {
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(rq: &HttpRequest, _pl: &mut Payload) -> Self::Future {
        let conn = rq.connection_info().clone();

        #[cfg(not(debug_assertions))]
        const SCHEMA: &str = "https";
        #[cfg(debug_assertions)]
        const SCHEMA: &str = "http";

        Box::pin(async move { Ok(Self(format!("{SCHEMA}://{}", conn.host()))) })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ClientIp {
    v4: [u8; 4],
}

impl ClientIp {
    pub fn new_v4(ipv4: &str) -> Result<Self, AppErr> {
        let mut ci = Self { v4: [0; 4] };
        for (i, x) in ipv4.split('.').take(4).enumerate() {
            let Ok(v) = x.parse::<u8>() else {
                return crate::err!(InvalidIpv4);
            };
            ci.v4[i] = v;
        }
        Ok(ci)
    }
    #[allow(dead_code)]
    pub fn v4(&self) -> [u8; 4] {
        self.v4
    }
}

impl FromRequest for ClientIp {
    type Error = AppErr;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(rq: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let conn = rq.connection_info();
        let ip = conn.realip_remote_addr().map(|v| v.to_string());
        Box::pin(async move {
            let Some(ip) = ip else {
                return crate::err!(ServerError, "no ip");
            };

            ClientIp::new_v4(&ip)
        })
    }
}
