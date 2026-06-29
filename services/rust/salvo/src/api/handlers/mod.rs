mod delete;
mod get;
mod patch;
mod post;
mod put;

pub use delete::delete_handler;
pub use get::get_handler;
pub use patch::patch_handler;
pub use post::post_handler;
pub use put::put_handler;

use crate::{
    state::State,
    task::{PatchedTask, Task},
};
use salvo::Depot;
use std::sync::{Arc, Mutex};
use tracing::error;
use uuid::Uuid;

pub fn state(depot: &mut Depot, method: &str) -> Arc<Mutex<State>> {
    match depot.obtain::<Arc<Mutex<State>>>().cloned() {
        Ok(state) => state,
        Err(_) => {
            error!(method, "Server State not injected into router at startup");
            panic!("Server State not injected into router at startup");
        }
    }
}

pub fn task_id(depot: &mut Depot, method: &str) -> Uuid {
    match depot.obtain::<Uuid>() {
        Ok(id) => *id,
        Err(_) => {
            error!(method, "Uuid not injected by validate_request middleware");
            panic!("Uuid not injected by validate_request middleware");
        }
    }
}

pub fn task(depot: &mut Depot, method: &str) -> Task {
    match depot.obtain::<Task>() {
        Ok(task) => task.to_owned(),
        Err(_) => {
            error!(method, "Task not injected by validate_request middleware");
            panic!("Task not injected by validate_request middleware");
        }
    }
}

pub fn patched_task(depot: &mut Depot, method: &str) -> PatchedTask {
    match depot.obtain::<PatchedTask>() {
        Ok(task) => task.to_owned(),
        Err(_) => {
            error!(
                method,
                "PatchedTask not injected by validate_request middleware"
            );
            panic!("PatchedTask not injected by validate_request middleware");
        }
    }
}
