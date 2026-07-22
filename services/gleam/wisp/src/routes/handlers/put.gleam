import routes/error.{
  internal, invalid_json_body, require, send_error, task_not_found,
}
import state.{type AppState, NotFound, Timeout, get, insert}
import task.{encode_task, parse_task}
import wisp.{type Request, type Response}
import youid/uuid.{type Uuid}

pub fn put_handler(request: Request, state: AppState, id: Uuid) -> Response {
  use body <- require(wisp.read_body_bits(request), invalid_json_body)
  use new_task <- require(parse_task(body), invalid_json_body)

  case get(state, id) {
    Ok(_task) -> {
      use _ <- require(insert(state, id, new_task), internal)
      wisp.json_response(encode_task(new_task), 200)
    }
    Error(NotFound) -> send_error(task_not_found)
    Error(Timeout) -> send_error(internal)
  }
}
