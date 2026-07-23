import app/state.{type AppState, NotFound, Timeout, get_task, insert_task}
import app/task.{Task, task_to_json}
import app/task_parsers.{parse_patched_task}
import routes/error.{
  internal, invalid_json_body, require, send_error, task_not_found,
}
import routes/logs.{log_not_found, log_patched}
import wisp.{type Request, type Response}
import youid/uuid.{type Uuid}

pub fn patch_handler(request: Request, state: AppState, id: Uuid) -> Response {
  use body <- require(wisp.read_body_bits(request), invalid_json_body)
  use new_task <- require(parse_patched_task(body), invalid_json_body)

  case get_task(state, id) {
    Ok(existing) -> {
      let task = Task(..existing, operation: new_task.operation)
      use _ <- require(insert_task(state, id, task), internal)

      log_patched(id, request, existing, task)

      wisp.json_response(task_to_json(task), 200)
    }
    Error(NotFound) -> {
      log_not_found(id, request)
      send_error(task_not_found)
    }
    Error(Timeout) -> send_error(internal)
  }
}
