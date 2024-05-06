use crate::state::WebState;
use crate::types::OkJsonResponseResult;
use actix_web::web;
use actix_web::Responder;
use lib_tm::processes as tm_processes;
use lib_tm::tm;

// Get PID of all the processes running in system
#[actix_web::get("/get-all-processes")]
async fn get_all_processes(
    state: WebState,
) -> OkJsonResponseResult<lib_tm::processes::AllProcessOut, tm_processes::ProcessError> {
    let mut sys_info = state.empty_sysinfo();
    let mut process_control = lib_tm::processes::Watcher::new(&mut sys_info);

    let all_process = process_control.all_processes();

    OkJsonResponseResult::new(Ok(all_process))
}

#[actix_web::get("/get-process/{process_id}")]
async fn get_process(state: WebState, process_id: web::Path<tm::ProcessId>) -> impl Responder {
    let mut sys_info = state.empty_sysinfo();
    let mut process_control = lib_tm::processes::Watcher::new(&mut sys_info);

    let pid = tm::sysinfo::Pid::from(process_id.into_inner());
    let process_info = process_control
        .process_info(pid)
        .map(|info| serde_json::json!(info));

    OkJsonResponseResult::<_, &'static str>::new(Ok(process_info))
}
