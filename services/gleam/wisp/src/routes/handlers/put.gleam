import app/state.{type AppState, NotFound, Timeout, get_task, insert_task}
import app/task.{task_to_json}
import app/task_parsers.{parse_task}
import routes/error.{
  internal, invalid_json_body, require, send_error, task_not_found,
}
import routes/logs.{log_not_found, log_overwrote}
import wisp.{type Request, type Response}
import youid/uuid.{type Uuid}

pub fn put_handler(request: Request, state: AppState, id: Uuid) -> Response {
  use body <- require(wisp.read_body_bits(request), invalid_json_body)
  use new_task <- require(parse_task(body), invalid_json_body)

  case get_task(state, id) {
    Ok(task) -> {
      use _ <- require(insert_task(state, id, new_task), internal)

      log_overwrote(id, request, task, new_task)

      wisp.json_response(task_to_json(new_task), 200)
    }
    Error(NotFound) -> {
      log_not_found(id, request)
      send_error(task_not_found)
    }
    Error(Timeout) -> send_error(internal)
  }
}
