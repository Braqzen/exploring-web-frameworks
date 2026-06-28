use crate::{
    api::handlers::{
        delete::remove, get::fetch, patch::partial_update, post::insert, put::overwrite,
    },
    state::State,
};
use rocket::{Build, Rocket, routes};
use std::sync::{Arc, Mutex};

pub fn router(state: Arc<Mutex<State>>) -> Rocket<Build> {
    rocket::build()
        .manage(state)
        .mount("/", routes![insert])
        .mount("/", routes![fetch, overwrite, partial_update, remove])
}
