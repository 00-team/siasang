use actix_files as af;
use actix_multipart::form::MultipartFormConfig;
use actix_web::{
    App, HttpServer, middleware as mw,
    web::{Data, scope},
    web::{JsonConfig, PathConfig, QueryConfig, ServiceConfig},
};
pub use config::Config;
// use media::MediaConfig;

pub use models::AppErr;
use potk::Command;

use crate::state::{AdminSessions, SiasangDb};

mod admin;
mod api;
mod config;
mod docs;
mod models;
mod state;
mod utils;
mod web;

fn config_app(app: &mut ServiceConfig) {
    if cfg!(debug_assertions) {
        app.service(af::Files::new("/public", "./rah/public"));
        app.service(af::Files::new("/rah-assets", "./rah/dist/rah-assets"));

        app.service(af::Files::new("/static", "static"));
        // app.service(af::Files::new("/record", MediaConfig::RECORD_BASE));
    }

    app.service(docs::openapi_json).service(docs::rapidoc);
    app.service(
        scope("/api")
            // .service(api::user::router())
            // .service(api::products::router())
            // .service(api::verification::verification)
            .service(
                scope("/admin")
                    .service(admin::auth::router())
                    .service(admin::state::router()),
            ),
    );
    // // app.service(mcp::router());
    // // app.service(mcp::r_mcp).service(mcp::r_mcp_options);
    app.service(web::router());
}

#[derive(Default, potk::Command)]
enum Commands {
    #[default]
    None,
    Help,
    Master {
        pass: String,
    },
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::set_logger(&kaf::logger::MasterLogger).expect("logger");
    log::set_max_level(log::LevelFilter::Debug);

    match potk::command::<Commands>() {
        Commands::None => {}
        Commands::Help => {
            log::info!("help: {}", Commands::help());
            return Ok(());
        }
        Commands::Master { pass } => {
            let hashed = utils::hash_password(&pass).expect("failed to hash");
            log::info!("hash: {hashed}");
            return Ok(());
        }
    };

    Config::get();
    Config::create_dirs()?;

    macro_rules! errhand {
        ($ident:ident) => {
            $ident::default()
                .error_handler(|e, _| err!(r, BadRequest, e.to_string()).into())
        };
    }

    let data_ass = Data::new(AdminSessions::default());

    #[cfg(debug_assertions)]
    data_ass.insert(8888, "8888".to_string()).await;

    let data_db = Data::new(SiasangDb::default());
    // let ss = state::SeedSettings::read().await.expect("read settings");
    // let data_transactions = Data::new(SeedTransactions::default());
    // let data_spice_state = Data::new(SpiceState::default());

    let server = HttpServer::new(move || {
        App::new()
            .wrap(mw::Logger::new("%s %r %Ts"))
            .app_data(data_ass.clone())
            .app_data(data_db.clone())
            // .app_data(data_spice_state.clone())
            // .app_data(data_html.clone())
            // .app_data(data_taker.clone())
            // .app_data(data_analytics.clone())
            // .app_data(data_ssr_cache.clone())
            // .app_data(data_pool.clone())
            // .app_data(data_verification_state.clone())
            .app_data(errhand!(PathConfig))
            .app_data(errhand!(QueryConfig))
            .app_data(errhand!(JsonConfig))
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(200 * 1024 * 1024)
                    .memory_limit(200 * 1024 * 1024)
                    .error_handler(|e, _rq| {
                        log::error!("mpf: {e:#?}");
                        err!(r, BadRequest, e.to_string()).into()
                    }),
            )
            .configure(config_app)
        // .wrap(mw::from_fn(bridge::headx))
    });

    #[cfg(debug_assertions)]
    let server = server.bind(("0.0.0.0", Config::PORT)).unwrap().run();

    #[cfg(all(not(debug_assertions), unix))]
    let server = {
        use std::os::unix::fs::PermissionsExt;
        let path = format!("/usr/share/nginx/socks/{}.sock", opt::SITE);
        // let path = format!("{PATH}/{}", Config::get().nginx_sock);
        let server = server.bind_uds(&path).unwrap();
        let perms = std::fs::Permissions::from_mode(0o777);
        std::fs::set_permissions(&path, perms)?;
        server.run()
    };

    // let server = if cfg!(debug_assertions) {
    //     // server.bind(("127.0.0.1", Config::PORT)).unwrap()
    // } else {
    // }
    // .run();

    // let server_handle = server.handle();

    server.await
}
