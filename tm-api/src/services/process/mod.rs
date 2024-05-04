use crate::state::WebState;
use actix_web::dev::HttpServiceFactory;
use actix_web::{web, Responder};
use lib_tm::processes::AllProcessOut;
use serde_json::json;

pub fn get_scopes() -> impl HttpServiceFactory {
    web::scope("/processes").service(get_all_processes)
}

#[actix_web::get("/get-all-processes")]
async fn get_all_processes(state: WebState<'_>) -> Result<web::Json<AllProcessOut>, impl Responder> {
    Ok(web::json(todo!()))
}
