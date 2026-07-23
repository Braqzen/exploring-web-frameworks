import gleam/json
import wisp.{type Response}

pub opaque type AppError {
  AppError(status: Int, message: String)
}

pub const task_not_found = AppError(404, "Task not found")

pub const invalid_path = AppError(404, "Invalid path")

pub const invalid_method = AppError(405, "Invalid method")

pub const invalid_json_body = AppError(422, "Invalid body JSON")

pub const internal = AppError(500, "Internal server error")

pub fn send_error(error: AppError) -> Response {
  let body =
    json.object([#("error", json.string(error.message))])
    |> json.to_string

  wisp.json_response(body, error.status)
}

pub fn require(
  result: Result(a, e),
  error: AppError,
  next: fn(a) -> Response,
) -> Response {
  case result {
    Ok(value) -> next(value)
    Error(_) -> send_error(error)
  }
}
