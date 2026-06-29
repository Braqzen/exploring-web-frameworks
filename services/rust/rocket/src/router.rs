use crate::{
    api::handlers::{delete_handler, get_handler, patch_handler, post_handler, put_handler},
    state::State,
};
use rocket::{Build, Rocket, routes};
use std::sync::{Arc, Mutex};

pub fn router(state: Arc<Mutex<State>>) -> Rocket<Build> {
    rocket::build()
        .manage(state)
        .mount("/", routes![post_handler])
        .mount(
            "/",
            routes![get_handler, put_handler, patch_handler, delete_handler],
        )
}
