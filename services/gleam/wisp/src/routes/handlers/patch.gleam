import app/operation.{encode_operation}
import app/state.{type AppState, NotFound, Timeout, get, insert}
import app/task.{Task, encode_task, parse_patched_task}
import gleam/http
import gleam/string
import palabres
import routes/error.{
  internal, invalid_json_body, require, send_error, task_not_found,
}
import wisp.{type Request, type Response}
import youid/uuid.{type Uuid}

pub fn patch_handler(request: Request, state: AppState, id: Uuid) -> Response {
  use body <- require(wisp.read_body_bits(request), invalid_json_body)
  use new_task <- require(parse_patched_task(body), invalid_json_body)

  case get(state, id) {
    Ok(existing) -> {
      let task = Task(..existing, operation: new_task.operation)
      use _ <- require(insert(state, id, task), internal)

      palabres.info("Patched task")
      |> palabres.string("id", uuid.to_string(id))
      |> palabres.string("from_operation", encode_operation(existing.operation))
      |> palabres.string("to_operation", encode_operation(new_task.operation))
      |> palabres.string("method", http.method_to_string(request.method))
      |> palabres.int("secret", string.length(task.secret))
      |> palabres.log

      wisp.json_response(encode_task(task), 200)
    }
    Error(NotFound) -> {
      palabres.warning("Task not found")
      |> palabres.string("id", uuid.to_string(id))
      |> palabres.string("method", http.method_to_string(request.method))
      |> palabres.log

      send_error(task_not_found)
    }
    Error(Timeout) -> send_error(internal)
  }
}
