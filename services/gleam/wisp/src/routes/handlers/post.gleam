import state.{type AppState}
import wisp.{type Request, type Response}
import youid/uuid.{type Uuid}

pub fn post_handler(request: Request, state: AppState) -> Response {
  wisp.ok()
}
