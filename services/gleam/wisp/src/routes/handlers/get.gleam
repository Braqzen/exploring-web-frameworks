import app/operation.{encode_operation}
import app/state.{type AppState, NotFound, Timeout, get}
import app/task.{encode_task}
import gleam/http
import gleam/string
import palabres
import routes/error.{internal, send_error, task_not_found}
import wisp.{type Request, type Response}
import youid/uuid.{type Uuid}

pub fn get_handler(request: Request, state: AppState, id: Uuid) -> Response {
  case get(state, id) {
    Ok(task) -> {
      palabres.info("Retrieved task")
      |> palabres.string("id", uuid.to_string(id))
      |> palabres.string("operation", encode_operation(task.operation))
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
