use actix_web::{web, App, HttpServer};

mod scopes;
mod services;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // initilize global state
            .app_data(web::Data::new(state::State::default()))
            //
            .service(web::scope("/v1").service(scopes::get_v1_scope()))
            // placeholder for future updates
            .service(web::scope("/v2").service(scopes::get_v2_scope()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
