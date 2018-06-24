use failure::Error;
use rocket_contrib::{Json, Value};
use crate::IotaCommand;

pub fn find_transactions(request: &Json<IotaCommand>) -> Result<Json<Value>, Error> {
    match &request.addresses {
        Some(addresses) => Ok(Json(json!(addresses))),
        None => Err(format_err!("No addresses provided")),
    }
}

pub fn get_transactions_to_approve(request: &Json<IotaCommand>) -> Result<Json<Value>, Error> {
    match &request.addresses {
        Some(addresses) => Ok(Json(json!(addresses))),
        None => Err(format_err!("No hashes provided")),
    }
}

pub fn broadcast_transactions(request: &Json<IotaCommand>) -> Result<Json<Value>, Error> {
    match &request.addresses {
        Some(addresses) => Ok(Json(json!(addresses))),
        None => Err(format_err!("No hashes provided")),
    }
}

pub fn store_transactions(request: &Json<IotaCommand>) -> Result<Json<Value>, Error> {
    match &request.addresses {
        Some(addresses) => Ok(Json(json!(addresses))),
        None => Err(format_err!("No hashes provided")),
    }
}

pub fn attach_to_tangle(request: &Json<IotaCommand>) -> Result<Json<Value>, Error> {
    match &request.addresses {
        Some(addresses) => Ok(Json(json!(addresses))),
        None => Err(format_err!("No hashes provided")),
    }
}