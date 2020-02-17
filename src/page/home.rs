use super::ViewPage;
use crate::entity::{Employee, ErrorMessage};
use crate::request::request;
use crate::{loading, logger, GMsg};
use seed::{prelude::*, *};

#[derive(Default)]
pub struct Model {
    employees: Status<Vec<Employee>>,
    employee_picked: Option<Employee>,
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

#[derive(Clone)]
pub enum Msg {
    ListLoaded(Result<Vec<Employee>, Vec<ErrorMessage>>),
    PickEmployee,
    EmployeePicked(Result<(Vec<Employee>, Option<Employee>), Vec<ErrorMessage>>),
    SlowLoadThresholdPassed,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::ListLoaded(Ok(employees)) => {
            model.employees = Status::Loaded(employees);
        }
        Msg::ListLoaded(Err(errors)) => {
            model.employees = Status::Failed;
            logger::errors(errors);
        }
        Msg::PickEmployee => {
            orders
                .skip()
                .perform_cmd(request::pick_employee(Msg::EmployeePicked));
        }
        Msg::EmployeePicked(Ok((employees, employee))) => {
            model.employees = Status::Loaded(employees);
            model.employee_picked = employee;
        }
        Msg::EmployeePicked(Err(errors)) => {
            model.employees = Status::Failed;
            model.employee_picked = None;
            logger::errors(errors);
        }
        Msg::SlowLoadThresholdPassed => {
            if let Status::Loading = model.employees {
                model.employees = Status::LoadingSlowly
            }
        }
    }
}

pub fn view<'a, Ms>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("Home", view_content(model))
}

fn view_content(model: &Model) -> Node<Msg> {
    match &model.employees {
        Status::Loading => empty![],
        Status::LoadingSlowly => loading::view_icon(),
        Status::Failed => loading::view_error("employees"),
        Status::Loaded(employees) => {
            let filter_fn: Box<dyn Fn(&Employee) -> bool> =
                if employees.iter().all(|employee| employee.picked) {
                    Box::new(|_| true)
                } else {
                    Box::new(|employee: &Employee| !employee.picked)
                };
            div![
                class!["columns is-vcentered"],
                table![
                    class!["table column"],
                    thead![tr![th!["They can be the next :D"],],],
                    tbody![employees.iter().filter_map(|employee| {
                        if filter_fn(employee) {
                            Some(tr![td![employee.name]])
                        } else {
                            None
                        }
                    })],
                ],
                button![
                    class!["button is-large is-primary column"],
                    simple_ev(Ev::Click, Msg::PickEmployee),
                    "Pick the next!"
                ],
                div![
                    class!["column"],
                    match &model.employee_picked {
                        Some(employee) => {
                            span!["Congratulations ", b![employee.name], ". You are the next!",]
                        }
                        None => {
                            empty![]
                        }
                    }
                ]
            ]
        }
    }
}
