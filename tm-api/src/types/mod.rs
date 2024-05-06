use actix_web::{web, HttpRequest, HttpResponse, Responder};

#[repr(transparent)]
pub struct OkJsonResponseResult<Ok, Err>(Result<Ok, Err>);

impl<Ok, Err> OkJsonResponseResult<Ok, Err> {
    pub fn as_ref(&self) -> &Result<Ok, Err> {
        &self.0
    }

    pub fn take(self) -> Result<Ok, Err> {
        self.0
    }
}

impl<Ok, Err> OkJsonResponseResult<Ok, Err>
where
    Ok: serde::ser::Serialize,
    Err: ToString,
{
    pub fn new(result: Result<Ok, Err>) -> Self {
        OkJsonResponseResult(result)
    }
}

impl<Ok, Err> Responder for OkJsonResponseResult<Ok, Err>
where
    Ok: serde::ser::Serialize,
    // Err is only boun to ToString,
    // with assumption that error will always be simple error message
    // if absolutely needed, wrap the whole error struct into debug string
    //
    // assumed example:
    // Error: "permission denied"
    //
    // If some complex additional field is required:
    // Error: format!("{:#?}", error_object)
    Err: ToString,
{
    type Body = <web::Json<String> as Responder>::Body;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        // we just need to convert this Object to appropriate type
        // actual response creation will be propagated
        let res_json_str = match self.0 {
            Ok(object) => {
                serde_json::json! ({
                    "result": object,
                })
            }
            Err(err) => {
                serde_json::json!( {
                    "error": err.to_string()
                })
            }
        }
        .to_string();

        <web::Json<String> as Responder>::respond_to(web::Json(res_json_str), req)
    }
}
