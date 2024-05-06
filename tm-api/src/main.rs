use actix_web::{middleware::Logger, web, App, HttpServer};

mod scopes;
mod services;
mod state;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    HttpServer::new(|| {
        let app_state = state::State::default();

        App::new()
            // wrap log to see every request
            .wrap(Logger::default())
            // bind global app state
            .app_data(web::Data::new(app_state))
            //
            .service(web::scope("/v1").service(scopes::get_v1_scope()))
            // placeholder for future updates
            .service(web::scope("/v2").service(scopes::get_v2_scope()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
