use crate::api::guard::error_json;
use rocket::{Request, catch, serde::json::Json};
use serde_json::Value;

#[catch(400)]
pub fn bad_request(req: &Request) -> Json<Value> {
    error_json(req)
}

#[catch(413)]
pub fn too_large(req: &Request) -> Json<Value> {
    error_json(req)
}

#[catch(422)]
pub fn unprocessable(req: &Request) -> Json<Value> {
    error_json(req)
}
