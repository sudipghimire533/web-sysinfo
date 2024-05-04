use actix_web::dev::HttpServiceFactory;

use super::services;

pub fn get_v1_scope() -> impl HttpServiceFactory {
    actix_web::web::scope("/v1").service(services::process::get_scopes())
}

pub fn get_v2_scope() -> impl HttpServiceFactory {
    actix_web::web::scope("/v2")
}
