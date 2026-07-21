import routes/error.{internal, send_error, task_not_found}
import state.{type AppState, NotFound, Timeout, get}
import task.{encode_task}
import wisp.{type Request, type Response}
import youid/uuid.{type Uuid}

pub fn get_handler(_request: Request, state: AppState, id: Uuid) -> Response {
  case get(state, id) {
    Ok(task) -> wisp.json_response(encode_task(task), 200)
    Error(NotFound) -> send_error(task_not_found)
    Error(Timeout) -> send_error(internal)
  }
}
