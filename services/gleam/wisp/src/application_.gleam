import gleam/http.{Delete, Get, Patch, Post, Put}
import routes/handlers.{
  delete_handler, get_handler, patch_handler, post_handler, put_handler,
}
import state.{type AppState}
import wisp.{type Request, type Response}
import youid/uuid.{type Uuid}

// Note: Erlang has some module named application therefore it is reserved and we must use application_.gleam

pub type Application {
  Application(state: AppState)
}

pub fn router(app: Application, request: Request) -> Response {
  case wisp.path_segments(request), request.method {
    [], Post -> post_handler(request, app.state)
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
    _, _ -> wisp.not_found()
  }
}

fn require_uuid(id: String, next: fn(Uuid) -> Response) -> Response {
  case uuid.from_string(id) {
    Ok(id) ->
      case uuid.version(id) {
        uuid.V4 -> next(id)
        _ -> wisp.bad_request("Invalid path")
      }
    Error(_) -> wisp.bad_request("Invalid path")
  }
}
