use crate::{entity::ErrorMessage, logger};
use seed::fetch;
use std::fmt::Debug;

pub mod request;

static BASE_API_URL: &str = "https://franz_strudel-caliaconf.builtwithdark.com";
const TIMEOUT: u32 = 5000;

pub fn new(path: &str) -> fetch::Request {
    fetch::Request::new(format!("{}/{}", BASE_API_URL, path)).timeout(TIMEOUT)
}

pub fn fail_reason_into_errors<T: Debug>(fail_reason: fetch::FailReason<T>) -> Vec<ErrorMessage> {
    match fail_reason {
        fetch::FailReason::RequestError(request_error, _) => {
            logger::error(request_error);
            vec!["Request error".into()]
        }
        fetch::FailReason::DataError(data_error, _) => {
            logger::error(data_error);
            vec!["Data error".into()]
        }
        fetch::FailReason::Status(_, fetch_object) => {
            // response isn't ok, but maybe contains error messages - try to decode them:
            match fetch_object.result.unwrap().data {
                Err(fetch::DataError::SerdeError(serde_error, json)) => {
                    logger::error(serde_error);
                    vec!["Data error".into()]
                }
                data => {
                    logger::error(data);
                    vec!["Data error".into()]
                }
            }
        }
    }
}
