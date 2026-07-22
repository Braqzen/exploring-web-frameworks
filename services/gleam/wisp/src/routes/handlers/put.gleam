import gleam/http
import gleam/string
import operation.{encode_operation}
import palabres
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
    Ok(task) -> {
      use _ <- require(insert(state, id, new_task), internal)

      palabres.info("Overwrote task")
      |> palabres.string("id", uuid.to_string(id))
      |> palabres.string("from_operation", encode_operation(task.operation))
      |> palabres.string("to_operation", encode_operation(new_task.operation))
      |> palabres.string("method", http.method_to_string(request.method))
      |> palabres.int("from_secret", string.length(task.secret))
      |> palabres.int("to_secret", string.length(new_task.secret))
      |> palabres.log

      wisp.json_response(encode_task(new_task), 200)
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
