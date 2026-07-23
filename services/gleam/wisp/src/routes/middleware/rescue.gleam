import exception
import routes/error.{internal, send_error}
import wisp.{type Response}

pub fn rescue_middleware(next: fn() -> Response) -> Response {
  case exception.rescue(next) {
    Ok(response) -> response
    Error(_) -> send_error(internal)
  }
}
