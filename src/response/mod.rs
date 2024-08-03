pub mod route;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum ResponseStatus {
    ///Request could be processed as expected.
    Ok,
    ///URL string is invalid.
    InvalidUrl,
    ///Service name is invalid.
    InvalidService,
    ///Version is not found.
    InvalidVersion,
    ///Options are invalid.
    InvalidOptions,
    ///The query string is synctactically malformed.
    InvalidQuery,
    ///The successfully parsed query parameters are invalid.
    InvalidValue,
    ///One of the supplied input coordinates could not snap to street segment.
    NoSegment,
    ///The request size violates one of the service specific request size restrictions.
    TooBig,
}
