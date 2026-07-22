import gleam/http
import palabres
import wisp.{type Request, type Response}

pub fn log_middleware(request: Request, next: fn() -> Response) -> Response {
  palabres.debug("Incoming request")
  |> palabres.string("method", http.method_to_string(request.method))
  |> palabres.string("path", request.path)
  |> palabres.log

  next()
}
