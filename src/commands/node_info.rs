use failure::Error;
use rocket_contrib::{Json, Value};

use crate::VERSION;

pub fn get_node_info() -> Result<Json<Value>, Error> {
    Ok(Json(json!({
       "appName": "Whit",
       "appVersion": VERSION,
       "jreAvailableProcesses": 0,
       "jreFreeMemory": 0,
       "jreMaxMemory": 0,
       "jreTotalMemory": 0,
       "latestMilestone": "",
       "latestMilestoneIndex": 0,
       "latestSolidSubtangleMilestone": "",
       "latestSolidSubtangleMilestoneIndex": "",
       "neighbors": 0,
       "packetsQueueSize": 0,
       "time": 0,
       "tips": 0,
       "transactionsToRequest": "",
   })))
}