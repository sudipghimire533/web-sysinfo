use actix_web::dev::HttpServiceFactory;

use super::services;

pub fn get_v1_scope() -> impl HttpServiceFactory {
    actix_web::web::scope("/sysinfo")
        .service(services::process::get_scopes())
        .service(services::system_specs::get_scopes())
}

pub fn get_v2_scope() -> impl HttpServiceFactory {
    actix_web::web::scope("/test")
}
