#[potk::enum_int(u16)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SiasangError {
    #[default]
    Unknown = 0,

    BadRequest, // 400
    InvalidIpv4,

    Forbidden, // 403
    BadAuth,

    NotFound, // 404

    Conflict, // 409

    TooManyRequests, // 429

    InternalServerError, // 500
    ServerError,

    ServiceUnavailable, // 503
}

impl SiasangError {
    pub const fn status(self) -> u16 {
        const UNKNOWN: u16 = SiasangError::Unknown as u16;
        const BAD_REQUEST: u16 = SiasangError::BadRequest as u16;
        const FORBIDDEN: u16 = SiasangError::Forbidden as u16;
        const NOT_FOUND: u16 = SiasangError::NotFound as u16;
        const CONFLICT: u16 = SiasangError::Conflict as u16;
        const TOO_MANY: u16 = SiasangError::TooManyRequests as u16;
        const INTERNAL: u16 = SiasangError::InternalServerError as u16;
        const UNAVAILABLE: u16 = SiasangError::ServiceUnavailable as u16;

        match self as u16 {
            UNKNOWN => 500,
            BAD_REQUEST..FORBIDDEN => 400,
            FORBIDDEN..NOT_FOUND => 403,
            NOT_FOUND..CONFLICT => 404,
            CONFLICT..TOO_MANY => 409,
            TOO_MANY..INTERNAL => 429,
            INTERNAL..UNAVAILABLE => 500,
            UNAVAILABLE.. => 503,
        }
    }
}
