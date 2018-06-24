use crate::IotaCommand;
use failure::Error;
use rocket_contrib::{Json, Value};

pub fn get_balances(request: &IotaCommand) -> Result<Json<Value>, Error> {
    match &request.addresses {
        Some(addresses) => Ok(Json(json!(addresses))),
        None => Err(format_err!("No hashes provided")),
    }
}

pub fn were_addresses_spent_from(request: &IotaCommand) -> Result<Json<Value>, Error> {
    match &request.addresses {
        Some(addresses) => Ok(Json(json!(addresses))),
        None => Err(format_err!("No hashes provided")),
    }
}
