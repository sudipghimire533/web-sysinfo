use crate::{state::WebState, types::OkJsonResponseResult};
use actix_web::{get, web};
use lib_tm::system_specs as tm_system_specs;

#[derive(Debug, thiserror::Error)]
pub enum SystemSpecsError {}

#[get("/get-system-specs")]
async fn get_system_specs(
    state: WebState,
) -> OkJsonResponseResult<tm_system_specs::SystemSpecs, SystemSpecsError> {
    let mut sysinfo = state.empty_sysinfo();
    let mut watcher = tm_system_specs::Watcher::new(&mut sysinfo);

    let system_specs = watcher.get_system_specs();
    OkJsonResponseResult::new(Ok(system_specs))
}
