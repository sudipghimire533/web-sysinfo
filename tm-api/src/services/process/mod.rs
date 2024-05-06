use actix_web::dev::HttpServiceFactory;
use actix_web::web;

mod get_all_processes;

// Bind all the processes call together
pub fn get_scopes() -> impl HttpServiceFactory {
    web::scope("/processes")
        .service(get_all_processes::get_all_processes)
        .service(get_all_processes::get_process)
}
