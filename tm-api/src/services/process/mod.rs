use crate::state::WebState;
use actix_web::dev::HttpServiceFactory;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponseBuilder, Responder};

// Bind all the processes call together
pub fn get_scopes() -> impl HttpServiceFactory {
    web::scope("/processes").service(get_all_processes)
}

// Get PID of all the processes running in system
#[actix_web::get("/get-all-processes")]
async fn get_all_processes(state: WebState) -> impl Responder {
    let mut sys_info = state.empty_sysinfo();
    let mut process_control = lib_tm::processes::Watcher::new(&mut sys_info);

    let all_process = process_control.all_processes();

    let request_status = StatusCode::OK;
    HttpResponseBuilder::new(request_status).json(serde_json::json!(
        {
            "status": request_status.as_u16(),
            "result": {
                "processes": all_process
            },
        }
    ))
}
