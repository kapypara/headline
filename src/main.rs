use actix_web::{App, HttpServer, middleware, web};

mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    // std::env::set_var("RUST_LOG", "info");
    // std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();

    log::info!("starting HTTP server at http://localhost:8080");

    let state = web::Data::new(app::headline_state().await);
    let pool = web::Data::new(app::headline_database().await);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(app::headline)
            .app_data(state.clone())
            .app_data(pool.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await


}
