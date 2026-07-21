import gleam/json
import routes/error.{internal, invalid_json_body, require}
import state.{type AppState, insert}
import task.{parse_task}
import wisp.{type Request, type Response}
import youid/uuid

pub fn post_handler(request: Request, state: AppState) -> Response {
  use body <- require(wisp.read_body_bits(request), invalid_json_body)
  use task <- require(parse_task(body), invalid_json_body)
  let id = uuid.v4()
  use _ <- require(insert(state, id, task), internal)

  wisp.json_response(
    json.object([#("id", json.string(uuid.to_string(id)))])
      |> json.to_string,
    201,
  )
}
