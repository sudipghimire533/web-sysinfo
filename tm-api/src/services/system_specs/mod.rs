pub mod get_system_specs;

// Bind all the processes call together
pub fn get_scopes() -> impl actix_web::dev::HttpServiceFactory {
    actix_web::web::scope("/system-specs").service(get_system_specs::get_system_specs)
}
