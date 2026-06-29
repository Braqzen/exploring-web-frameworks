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

use crate::state::State;
use salvo::{Depot, Request};
use std::sync::{Arc, Mutex};
use tracing::error;
use uuid::Uuid;

pub fn state(depot: &mut Depot) -> &Arc<Mutex<State>> {
    match depot.obtain::<Arc<Mutex<State>>>() {
        Ok(state) => state,
        Err(_) => {
            error!("Server State not injected into router at startup");
            panic!("Server State not injected into router at startup");
        }
    }
}

pub fn task_id(request: &Request) -> Option<Uuid> {
    request.param::<Uuid>("task_id")
}
