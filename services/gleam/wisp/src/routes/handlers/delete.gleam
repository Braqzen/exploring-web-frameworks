import routes/error.{internal, send_error, task_not_found}
import state.{type AppState, NotFound, Timeout, delete}
import wisp.{type Request, type Response}
import youid/uuid.{type Uuid}

pub fn delete_handler(_request: Request, state: AppState, id: Uuid) -> Response {
  case delete(state, id) {
    Ok(_) -> wisp.no_content()
    Error(NotFound) -> send_error(task_not_found)
    Error(Timeout) -> send_error(internal)
  }
}
