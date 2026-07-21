import gleam/dict.{type Dict}
import task.{type Task}
import youid/uuid.{type Uuid}

// TODO: config
pub type AppState {
  AppState(tasks: Dict(Uuid, Task))
}

pub fn new_app_state() -> AppState {
  AppState(tasks: dict.new())
}
