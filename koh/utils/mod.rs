mod external;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
#[allow(unused_imports)]
pub use external::*;

pub fn hash_password(
    password: &str,
) -> Result<String, argon2::password_hash::Error> {
    let hash = Argon2::default().hash_password(password.as_bytes())?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, stored: &str) -> bool {
    let Ok(ph) = PasswordHash::new(stored) else { return false };
    Argon2::default().verify_password(password.as_bytes(), &ph).is_ok()
}

// pub fn slug_check(slug: &str) -> Result<(), AppErr> {
//     if slug.len() > Config::SLUG_MAX {
//         return anbar::err!(BadSlug);
//     }
//
//     for ch in slug.chars() {
//         let valid = ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-';
//         if !valid {
//             return anbar::err!(BadSlug);
//         }
//     }
//
//     Ok(())
// }

// pub trait CutOff {
//     fn cut_off(&mut self, len: usize);
// }
//
// impl CutOff for String {
//     fn cut_off(&mut self, len: usize) {
//         let mut idx = len;
//         loop {
//             if self.is_char_boundary(idx) {
//                 break;
//             }
//             idx -= 1;
//         }
//         self.truncate(idx)
//     }
// }
//
// impl CutOff for Option<String> {
//     fn cut_off(&mut self, len: usize) {
//         let Some(v) = self else { return };
//         v.cut_off(len);
//     }
// }

#[macro_export]
macro_rules! upt {
    ($item:ident, $from:ident, $var:ident) => {
        if let Some(v) = $from.$var {
            $item.$var = v;
        }
    };
    ($item:ident, $from:ident, $var:ident, into) => {
        if let Some(v) = $from.$var {
            $item.$var = v.into();
        }
    };
    ($item:ident, $from:ident, $var:ident, $ty:ident) => {
        if let Some(v) = $from.$var {
            $item.$var = v as $ty;
        }
    };
}
