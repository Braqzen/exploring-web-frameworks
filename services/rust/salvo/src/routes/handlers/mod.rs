mod delete;
mod get;
mod invalid_method;
mod invalid_path;
mod patch;
mod post;
mod put;

pub use delete::delete_handler;
pub use get::get_handler;
pub use invalid_method::invalid_method_handler;
pub use invalid_path::invalid_path_handler;
pub use patch::patch_handler;
pub use post::post_handler;
pub use put::put_handler;

use crate::state::AppState;
use salvo::Depot;
use std::sync::{Arc, Mutex};
use tracing::error;

pub fn state(depot: &mut Depot, method: &str) -> Arc<Mutex<AppState>> {
    match depot.obtain::<Arc<Mutex<AppState>>>().cloned() {
        Ok(state) => state,
        Err(_) => {
            error!(method, "Server State not injected into router at startup");
            panic!("Server State not injected into router at startup");
        }
    }
}
