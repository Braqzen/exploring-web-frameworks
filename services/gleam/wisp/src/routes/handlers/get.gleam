import app/state.{type AppState, NotFound, Timeout, get_task}
import app/task.{task_to_json}
import routes/error.{internal, send_error, task_not_found}
import routes/logs.{log_not_found, log_retrieved}
import wisp.{type Request, type Response}
import youid/uuid.{type Uuid}

pub fn get_handler(request: Request, state: AppState, id: Uuid) -> Response {
  case get_task(state, id) {
    Ok(task) -> {
      log_retrieved(id, request, task)
      wisp.json_response(task_to_json(task), 200)
    }
    Error(NotFound) -> {
      log_not_found(id, request)
      send_error(task_not_found)
    }
    Error(Timeout) -> send_error(internal)
  }
}
