import app/operation.{encode_operation}
import app/state.{type AppState, insert}
import app/task.{parse_task}
import gleam/http
import gleam/json
import gleam/string
import palabres
import routes/error.{internal, invalid_json_body, require}
import wisp.{type Request, type Response}
import youid/uuid

pub fn post_handler(request: Request, state: AppState) -> Response {
  use body <- require(wisp.read_body_bits(request), invalid_json_body)
  use task <- require(parse_task(body), invalid_json_body)
  let id = uuid.v4()
  use _ <- require(insert(state, id, task), internal)

  palabres.info("Inserted new task")
  |> palabres.string("id", uuid.to_string(id))
  |> palabres.string("operation", encode_operation(task.operation))
  |> palabres.string("method", http.method_to_string(request.method))
  |> palabres.int("secret", string.length(task.secret))
  |> palabres.log

  wisp.json_response(
    json.object([#("id", json.string(uuid.to_string(id)))])
      |> json.to_string,
    201,
  )
}
