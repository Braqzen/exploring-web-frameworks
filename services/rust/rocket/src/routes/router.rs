use crate::routes::{
    catchers::{internal_error, method_not_allowed, not_found, unprocessable},
    fairings::LogFairing,
    handlers::{delete_handler, get_handler, patch_handler, post_handler, put_handler},
};
use app::state::AppState;
use rocket::{
    Build, Config, Rocket, catchers,
    data::{Limits, ToByteUnit},
    routes,
};
use std::sync::{Arc, Mutex};

/// The multipler for the maximum size of a request body
const BYTES: usize = 1024;

pub fn router(state: Arc<Mutex<AppState>>) -> Rocket<Build> {
    // SAFETY: Nothing should have locked on boot therefore cannot panic
    let max_size = BYTES * state.lock().unwrap().config.request_size_limit as usize;

    let limit_figment =
        Config::figment().merge(("limits", Limits::default().limit("json", max_size.bytes())));

    rocket::custom(limit_figment)
        .manage(state)
        .mount("/", routes![post_handler])
        .mount(
            "/",
            routes![get_handler, put_handler, patch_handler, delete_handler],
        )
        .register(
            "/",
            catchers![unprocessable, not_found, method_not_allowed, internal_error],
        )
        .attach(LogFairing)
}
