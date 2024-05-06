use crate::state::WebState;
use actix_web::web;
use actix_web::{http::StatusCode, HttpResponseBuilder, Responder};
use lib_tm::tm;
use serde_json::json;

// Get PID of all the processes running in system
#[actix_web::get("/get-all-processes")]
async fn get_all_processes(state: WebState) -> impl Responder {
    let mut sys_info = state.empty_sysinfo();
    let mut process_control = lib_tm::processes::Watcher::new(&mut sys_info);

    let all_process = process_control.all_processes();

    let request_status = StatusCode::OK;
    HttpResponseBuilder::new(request_status).json(json!(
        {
            "status": request_status.as_u16(),
            "result": {
                "processes": all_process
            },
        }
    ))
}

#[actix_web::get("/get-process/{process_id}")]
async fn get_process(state: WebState, process_id: web::Path<tm::ProcessId>) -> impl Responder {
    let mut sys_info = state.empty_sysinfo();
    let mut process_control = lib_tm::processes::Watcher::new(&mut sys_info);

    let pid = tm::sysinfo::Pid::from(process_id.into_inner());
    let process_info: Result<_, anyhow::Error> = process_control.process_info(pid);

    let (status, body) = match process_info {
        Ok(process_info) => {
            let status = StatusCode::OK;
            let body = serde_json::json!(
                {
                    "status": status.as_u16(),
                    "result": process_info,
                }
            );
            (status, body)
        }
        Err(err) => {
            let status = StatusCode::BAD_REQUEST;
            let body = serde_json::json!(
                {
                    "status": status.as_u16(),
                    "error": format!("{err}"),
                }
            );
            (status, body)
        }
    };

    HttpResponseBuilder::new(status).json(body)
}
