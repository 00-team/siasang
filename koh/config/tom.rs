#[derive(Debug, serde::Deserialize)]
pub struct ConfigToml {
    pub master_hash: String,
}

pub fn get() -> ConfigToml {
    log::info!("reading toml config at: env.toml");

    let path = std::env::args()
        .skip(1)
        .find_map(|a| a.strip_prefix("--config=").map(String::from))
        .unwrap_or_else(|| "env.toml".to_string());

    let data = match std::fs::read_to_string(&path) {
        Ok(v) => v,
        Err(e) => panic!("could not read config at: {path}\n{e:#?}"),
    };

    match toml::from_str(&data) {
        Ok(v) => v,
        Err(e) => panic!("invalid toml config file: {path}\n{e}"),
    }
}
