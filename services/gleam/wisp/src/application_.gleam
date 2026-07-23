import app/state.{type AppState}
import gleam/http.{Delete, Get, Patch, Post, Put}
import routes/error.{invalid_method, invalid_path, send_error}
import routes/handlers.{
  delete_handler, get_handler, patch_handler, post_handler, put_handler,
}
import routes/middleware.{chaos_middleware, log_middleware, rescue_middleware}
import wisp.{type Request, type Response}
import youid/uuid.{type Uuid}

// Note: Erlang has some module named application therefore it is reserved and we must use application_.gleam

const bytes: Int = 1024

pub type Application {
  Application(state: AppState)
}

pub fn router(app: Application, request: Request) -> Response {
  let request =
    wisp.set_max_body_size(request, app.state.config.request_size_limit * bytes)

  use <- rescue_middleware
  use <- log_middleware(request)
  use <- chaos_middleware(request, app.state)

  case wisp.path_segments(request), request.method {
    [], Post -> post_handler(request, app.state)
    [], _ -> send_error(invalid_method)
    [id], Get -> {
      use id <- require_uuid(id)
      get_handler(request, app.state, id)
    }
    [id], Put -> {
      use id <- require_uuid(id)
      put_handler(request, app.state, id)
    }
    [id], Patch -> {
      use id <- require_uuid(id)
      patch_handler(request, app.state, id)
    }
    [id], Delete -> {
      use id <- require_uuid(id)
      delete_handler(request, app.state, id)
    }
    [_], _ -> send_error(invalid_method)
    _, _ -> send_error(invalid_path)
  }
}

fn require_uuid(id: String, next: fn(Uuid) -> Response) -> Response {
  case uuid.from_string(id) {
    Ok(id) ->
      case uuid.version(id) {
        uuid.V4 -> next(id)
        _ -> send_error(invalid_path)
      }
    Error(_) -> send_error(invalid_path)
  }
}
