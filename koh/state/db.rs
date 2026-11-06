use crate::models::Owj;
use std::collections::HashMap;

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct SiasangBackup {
    pub created_at: u64,
    pub size: u64,
    pub sha256: [u8; 32],
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct SiasangLevel {
    update_interval: UpdateInterval,
    pub backups: Vec<SiasangBackup>,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct UpdateInterval {
    /// its needs to be at least this many seconds before you can update the
    /// backup. so a bad actor wont just push 100 files a sec
    min: u64,
    // /// if an update does not happen with in this many seconds we send a notif
    // /// saying that service is probebly dead
    // max: u64,
    /// the average expected update. lets say 7 days
    /// if an update coms faster than 7days / 2 then something has gone wrong
    /// if an update coms longer than 7days * 1.5 then something is also wrong
    avg: u64,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct SiasangScope {
    levels: HashMap<String, SiasangLevel>,
}

#[derive(Default)]
#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct SiasangDb {
    updated_at: u64,
    scopes: HashMap<String, SiasangScope>,
    pub owjes: HashMap<u64, Owj>,
}

// #[derive(Debug, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
// #[serde(default)]
// pub struct SeedSettingJson {
//     #[schema(required)]
//     rob_irr: Vec<RobIrrPrice>,
//     #[schema(required)]
//     wallet: GoojeWallet,
//     #[schema(required)]
//     rob: RobState,
//     #[schema(required)]
//     rob_rewards: RobRewards,
//     #[schema(required)]
//     rob_prices: RobPrices,
// }
//
// impl SeedSettingJson {
//     pub async fn from_settings(value: &SeedSettings) -> Self {
//         Self {
//             rob_irr: value.rob_irr.lock().await.clone(),
//             wallet: *value.wallet.lock().await,
//             rob: *value.rob.lock().await,
//             rob_rewards: *value.rob_rewards.lock().await,
//             rob_prices: *value.rob_prices.lock().await,
//         }
//     }
// }
//
// impl From<SeedSettingJson> for SeedSettings {
//     fn from(value: SeedSettingJson) -> Self {
//         Self {
//             rob_irr: Mutex::new(value.rob_irr),
//             wallet: Mutex::new(value.wallet),
//             rob: Mutex::new(value.rob),
//             rob_rewards: Mutex::new(value.rob_rewards),
//             rob_prices: Mutex::new(value.rob_prices),
//         }
//     }
// }
//
// impl SeedSettings {
//     pub async fn rob_available(&self) -> u64 {
//         self.rob.lock().await.available
//     }
//
//     pub async fn rob_irr_price(&self) -> u64 {
//         let v = self.rob_irr.lock().await;
//         let Some(last) = v.last() else { return 0 };
//         last.irr
//     }
//
//     pub async fn rob_available_set(&self, rob: u64) {
//         self.rob.lock().await.available = rob;
//     }
//
//     pub async fn rob_irr_set(&self, irr: u64, user: Gene) {
//         let mut v = self.rob_irr.lock().await;
//         v.push(RobIrrPrice {
//             irr,
//             timestamp: soil::sys_now(),
//             user,
//         });
//     }
//
//     pub async fn rob_sold(&self, rob: u64) {
//         let mut rs = self.rob.lock().await;
//         rs.sold += rob;
//         rs.available = rs.available.saturating_sub(rob);
//     }
//
//     pub async fn wallet_add(&self, value: GoojeWallet) {
//         *self.wallet.lock().await += value;
//     }
//
//     pub async fn wallet_sub(&self, value: GoojeWallet) {
//         *self.wallet.lock().await -= value;
//     }
//
//     pub async fn rob_irr_rollback(&self) {
//         self.rob_irr.lock().await.pop();
//     }
//
//     // pub async fn update(&self, ssj: &SeedSettingJson) {
//     //     *self.rob_irr.lock().await = ssj.rob_irr;
//     // }
//
//     pub async fn save(&self) -> std::io::Result<SeedSettingJson> {
//         let json = SeedSettingJson::from_settings(self).await;
//         let cc = serde_json::to_string(&json)?;
//         tokio::fs::write(Config::SETTINGS_PATH, cc).await?;
//
//         Ok(json)
//     }
//
//     pub async fn read() -> std::io::Result<Self> {
//         let cc = tokio::fs::read_to_string(Config::SETTINGS_PATH).await;
//         let ssj = if let Ok(cc) = cc {
//             serde_json::from_str::<SeedSettingJson>(&cc).unwrap_or_default()
//         } else {
//             SeedSettingJson::default()
//         };
//
//         let ss = Self::from(ssj);
//         ss.save().await?;
//
//         Ok(ss)
//     }
// }
