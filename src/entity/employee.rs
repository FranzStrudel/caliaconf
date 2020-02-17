use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type EmployeeId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub uuid: EmployeeId,
    pub name: String,
    pub picked: bool,
}
