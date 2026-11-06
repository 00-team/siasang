use crate::AppErr;
use actix_web::HttpRequest;

macro_rules! auth {
    ($name:ident, $key:literal, {  $($field:ident : $ty:ty),* $(,)? }) => {
        pub struct $name { $(pub $field : $ty),* }

impl TryFrom<&HttpRequest> for $name {
    type Error = AppErr;

    fn try_from(rq: &HttpRequest) -> Result<Self, Self::Error> {
        if let Some(value) = rq.headers().get($key) {
            return $name::try_from(value.to_str()?);
        }

        for hdr in rq.headers().get_all("cookie") {
            let Ok(hv) = hdr.to_str() else { continue };
            for cookie in hv.split(';') {
                let mut s = cookie.splitn(2, '=');

                let Some(k) = s.next().map(|v| v.trim()) else { continue };
                let Some(v) = s.next().map(|v| v.trim()) else { continue };

                if k.to_lowercase() == $key {
                    if let Ok(v) = $name::try_from(v) {
                        return Ok(v);
                    }
                }
            }
        }

        crate::err!(BadAuth)
    }
}
    };
}

auth!(AuthAdmin, "x-siasang-auth-admin", { id: u64, token: String });
auth!(AuthOwj, "x-siasang-auth-owj", { id: u64, token: String });

impl TryFrom<&str> for AuthAdmin {
    type Error = AppErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let [id, token] = tokenizer(value)?;
        let Ok(id) = id.parse::<u64>() else {
            return crate::err!(BadAuth, "invalid id");
        };
        Ok(Self { id, token: token.to_string() })
    }
}

impl TryFrom<&str> for AuthOwj {
    type Error = AppErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let [id, token] = tokenizer(value)?;
        let Ok(id) = id.parse::<u64>() else {
            return crate::err!(BadAuth, "invalid id");
        };
        Ok(Self { id, token: token.to_string() })
    }
}

// impl TryFrom<&str> for AuthSsr {
//     type Error = AppErr;
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         let [pass] = tokenizer(value)?;
//         Ok(Self { pass: pass.to_string() })
//     }
// }

fn tokenizer<const N: usize>(value: &str) -> Result<[&str; N], AppErr> {
    let mut result: [&str; N] = [""; N];
    let mut v = value.splitn(N, ':');
    for slot in result.iter_mut() {
        let Some(s) = v.next() else {
            return crate::err!(BadAuth);
        };
        *slot = s;
    }

    Ok(result)
}
