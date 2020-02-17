use super::ViewPage;
use crate::entity::{Employee, EmployeeId, ErrorMessage};
use crate::request::request;
use crate::{entity, loading, logger, GMsg};
use enclose::enc;
use seed::{prelude::*, *};

const ENTER_KEY: u32 = 13;

#[derive(Default)]
pub struct Model {
    employees: Status<Vec<Employee>>,
    new_employee_name: String,
}

enum Status<T> {
    Loading,
    LoadingSlowly,
    Loaded(T),
    Failed,
}

impl<T> Default for Status<T> {
    fn default() -> Self {
        Self::Loading
    }
}

pub fn init(orders: &mut impl Orders<Msg, GMsg>) -> Model {
    orders
        .perform_cmd(loading::notify_on_slow_load(Msg::SlowLoadThresholdPassed))
        .perform_cmd(request::load_list(Msg::ListLoaded));

    Model::default()
}

pub enum Msg {
    ListLoaded(Result<Vec<Employee>, Vec<ErrorMessage>>),
    RemoveEmployee(EmployeeId),
    NewEmployeeNameChanged(String),
    AddEmployee,
    SlowLoadThresholdPassed,
    NoOp,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::ListLoaded(Ok(employees)) => {
            model.employees = Status::Loaded(employees);
            model.new_employee_name.clear();
        }
        Msg::ListLoaded(Err(errors)) => {
            model.employees = Status::Failed;
            model.new_employee_name.clear();
            logger::errors(errors);
        }
        Msg::RemoveEmployee(employee_id) => {
            orders
                .skip()
                .perform_cmd(request::remove_employee(employee_id, Msg::ListLoaded));
        }
        Msg::NewEmployeeNameChanged(name) => {
            model.new_employee_name = name;
        }
        Msg::AddEmployee => {
            orders.skip().perform_cmd(request::add_employee(
                model.new_employee_name.clone(),
                Msg::ListLoaded,
            ));
        }
        Msg::SlowLoadThresholdPassed => {
            if let Status::Loading = model.employees {
                model.employees = Status::LoadingSlowly
            }
        }
        Msg::NoOp => (),
    }
}

pub fn view<'a, Ms>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("Settings", view_content(model))
}

fn view_content(model: &Model) -> Node<Msg> {
    match &model.employees {
        Status::Loading => empty![],
        Status::LoadingSlowly => loading::view_icon(),
        Status::Failed => loading::view_error("employees"),
        Status::Loaded(employees) => div![
            class!["container"],
            table![
                class!["table"],
                thead![tr![td![
                    attrs! {At::ColSpan => 2},
                    input![
                        attrs! {At::Value => model.new_employee_name},
                        keyboard_ev(Ev::KeyDown, |keyboard_event| {
                            if keyboard_event.key_code() == ENTER_KEY {
                                Msg::AddEmployee
                            } else {
                                Msg::NoOp
                            }
                        }),
                        input_ev(Ev::Input, Msg::NewEmployeeNameChanged),
                    ],
                ],],],
                tbody![employees.iter().map(|employee| tr![
                    td![employee.name],
                    td![div![
                        class!["delete is-small"],
                        ev(
                            Ev::Click,
                            enc!((employee) move |_| Msg::RemoveEmployee(employee.uuid))
                        )
                    ]],
                ])],
            ],
        ],
    }
}
