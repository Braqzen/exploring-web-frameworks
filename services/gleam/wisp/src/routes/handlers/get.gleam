import state.{type AppState}
import wisp.{type Request, type Response}
import youid/uuid.{type Uuid}

pub fn get_handler(request: Request, state: AppState, id: Uuid) -> Response {
  wisp.ok()
}
