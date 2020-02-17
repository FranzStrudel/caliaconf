use crate::entity::{Employee, EmployeeId};
use crate::{entity::ErrorMessage, request};
use seed::fetch::ResponseDataResult;
use seed::Method;
use serde::{Deserialize, Serialize};

pub async fn load_list<Ms: 'static>(
    f: fn(Result<Vec<Employee>, Vec<ErrorMessage>>) -> Ms,
) -> Result<Ms, Ms> {
    request::new("list")
        .fetch_json_data(move |data_result: ResponseDataResult<Vec<Employee>>| {
            f(data_result.map_err(request::fail_reason_into_errors))
        })
        .await
}

pub async fn remove_employee<Ms: 'static>(
    employee_id: EmployeeId,
    f: fn(Result<Vec<Employee>, Vec<ErrorMessage>>) -> Ms,
) -> Result<Ms, Ms> {
    #[derive(Serialize)]
    struct Payload {
        employee_id: EmployeeId,
    };
    request::new("remove")
        .method(Method::Post)
        .send_json(&Payload { employee_id })
        .fetch_json_data(move |data_result: ResponseDataResult<Vec<Employee>>| {
            f(data_result.map_err(request::fail_reason_into_errors))
        })
        .await
}

pub async fn add_employee<Ms: 'static>(
    name: String,
    f: fn(Result<Vec<Employee>, Vec<ErrorMessage>>) -> Ms,
) -> Result<Ms, Ms> {
    #[derive(Serialize)]
    struct Payload {
        name: String,
    };
    request::new("add")
        .method(Method::Post)
        .send_json(&Payload { name })
        .fetch_json_data(move |data_result: ResponseDataResult<Vec<Employee>>| {
            f(data_result.map_err(request::fail_reason_into_errors))
        })
        .await
}

pub async fn pick_employee<Ms: 'static>(
    f: fn(Result<(Vec<Employee>, Option<Employee>), Vec<ErrorMessage>>) -> Ms,
) -> Result<Ms, Ms> {
    #[derive(Serialize)]
    request::new("pick")
        .method(Method::Get)
        .fetch_json_data(
            move |data_result: ResponseDataResult<(Vec<Employee>, Option<Employee>)>| {
                f(data_result.map_err(request::fail_reason_into_errors))
            },
        )
        .await
}
