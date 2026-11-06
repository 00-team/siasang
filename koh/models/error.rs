use actix_web::ResponseError;
use actix_web::{HttpResponse, body::BoxBody, http::StatusCode};
use kaf::SiasangError;
use tokio::task::JoinError;

impl From<SiasangError> for AppErr {
    fn from(value: SiasangError) -> Self {
        Self { status: value.status(), debug: None, code: value }
    }
}

impl From<JoinError> for AppErr {
    fn from(value: JoinError) -> Self {
        Self::from(SiasangError::ServerError)
            .debug(&format!("failed to join tokio task: {value:#?}"))
    }
}

#[derive(serde::Serialize, Debug, utoipa::ToSchema, Clone)]
pub struct AppErr {
    pub status: u16,
    pub code: SiasangError,
    pub debug: Option<String>,
}

impl AppErr {
    pub fn server_error() -> Self {
        Self { status: 500, code: SiasangError::ServerError, debug: None }
    }

    pub fn debug(mut self, debug: &str) -> Self {
        self.debug = Some(debug.to_string());
        self
    }
}

impl std::error::Error for AppErr {}

impl std::fmt::Display for AppErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppErr {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }
}

impl From<actix_web::http::header::ToStrError> for AppErr {
    fn from(value: actix_web::http::header::ToStrError) -> Self {
        // use actix_web::http::header::ToStrError
        Self {
            status: 400,
            code: SiasangError::BadRequest,
            debug: Some(format!(
                "could not convert this header value to string: {value:?}"
            )),
        }
    }
}

macro_rules! fse {
    ($ty:path) => {
        impl From<$ty> for AppErr {
            fn from(value: $ty) -> Self {
                log::error!("_500_: [{}] {value:?}", stringify!($ty));
                Self::server_error().debug(stringify!($ty))
            }
        }
    };
}

fse!(std::io::Error);
fse!(reqwest::Error);

#[macro_export]
macro_rules! err {
    ($code:ident) => {
        Err($crate::AppErr::from(kaf::SiasangError::$code))
    };

    ($code:ident, $debug:literal) => {
        Err($crate::AppErr::from(kaf::SiasangError::$code).debug($debug))
    };

    ($code:ident, $debug:expr) => {
        Err($crate::AppErr::from(kaf::SiasangError::$code).debug(&$debug))
    };

    (r, $code:ident) => {
        $crate::AppErr::from(kaf::SiasangError::$code)
    };

    (c, $code:ident) => {
        Err(kaf::SiasangError::$code)
    };

    (r,$code:ident, $debug:literal) => {
        $crate::AppErr::from(kaf::SiasangError::$code).debug($debug)
    };

    (r,$code:ident, $debug:expr) => {
        $crate::AppErr::from(kaf::SiasangError::$code).debug(&$debug)
    };
}
