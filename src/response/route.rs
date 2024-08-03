use serde::{Deserialize, Serialize};

use super::ResponseStatus;


#[derive(Debug, Serialize, Deserialize)]
pub struct RouteResponse {
    pub code: ResponseStatus,
    pub waypoints: Vec<Vec<u32>>,

}
