use std::sync::OnceLock;

mod tom;

#[derive(Debug)]
/// `koh` Config
pub struct Config {
    pub master_hash: String,
}

impl Config {
    pub const TITLE_MAX: usize = 255;
    pub const SLUG_MAX: usize = 255;
    pub const DESC_MAX: usize = 4096;

    pub const BKCODE_MIN: usize = 8;
    pub const BKCODE_MAX: usize = 16;
    pub const BKCODE_ABC: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

    // pub const VITE_MANIFEST: &str = "../web-app/dist/.vite/manifest.json";

    /// 180 days
    pub const TOKEN_LIFE: u64 = 180 * 24 * 3600;

    pub const API_VERSION: &str = "0.2.0";
    pub const PORT: u16 = 7000;

    /// zstd encode level
    pub const CMP_LVL: i32 = 3;

    pub const CODE_ABC: &[u8] = b"0123456789";
    pub const TOKEN_ABC: &[u8] =
        b"!@#$%^&*_+abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*_+";

    pub fn create_dirs() -> std::io::Result<()> {
        // MediaConfig::create_dirs()?;

        Ok(())
    }

    // fn ssr_client(ssr_pass: &str) -> reqwest::Client {
    //     let mut headers = reqwest::header::HeaderMap::new();
    //     headers.insert(
    //         reqwest::header::AUTHORIZATION,
    //         reqwest::header::HeaderValue::from_str(ssr_pass)
    //             .expect("invalid ssr header"),
    //     );
    //     reqwest::ClientBuilder::new()
    //         .default_headers(headers)
    //         .connection_verbose(false)
    //         .build()
    //         .expect("bad simurgh clinet")
    // }

    fn init() -> Self {
        let ct = tom::get();

        Self { master_hash: ct.master_hash }
    }

    pub fn get() -> &'static Self {
        static STATE: OnceLock<Config> = OnceLock::new();
        STATE.get_or_init(Self::init)
    }
}
