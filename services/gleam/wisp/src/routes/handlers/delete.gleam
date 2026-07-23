import app/state.{type AppState, NotFound, Timeout, delete_task}
import routes/error.{internal, send_error, task_not_found}
import routes/logs.{log_not_found, log_removed}
import wisp.{type Request, type Response}
import youid/uuid.{type Uuid}

pub fn delete_handler(request: Request, state: AppState, id: Uuid) -> Response {
  case delete_task(state, id) {
    Ok(task) -> {
      log_removed(id, request, task)
      wisp.no_content()
    }
    Error(NotFound) -> {
      log_not_found(id, request)
      send_error(task_not_found)
    }
    Error(Timeout) -> send_error(internal)
  }
}
