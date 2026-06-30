use crate::{
    api::{
        catchers::{bad_request, method_not_allowed, not_found, too_large, unprocessable},
        handlers::{delete_handler, get_handler, patch_handler, post_handler, put_handler},
    },
    state::State,
};
use rocket::{Build, Rocket, catchers, routes};
use std::sync::{Arc, Mutex};

pub fn router(state: Arc<Mutex<State>>) -> Rocket<Build> {
    rocket::build()
        .manage(state)
        .mount("/", routes![post_handler])
        .mount(
            "/",
            routes![get_handler, put_handler, patch_handler, delete_handler],
        )
        .register(
            "/",
            catchers![
                bad_request,
                too_large,
                unprocessable,
                not_found,
                method_not_allowed
            ],
        )
}
