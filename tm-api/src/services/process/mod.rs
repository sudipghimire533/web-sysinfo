mod get_all_processes;

// Bind all the processes call together
pub fn get_scopes() -> impl actix_web::dev::HttpServiceFactory {
    actix_web::web::scope("/processes")
        .service(get_all_processes::get_all_processes)
        .service(get_all_processes::get_process)
}
