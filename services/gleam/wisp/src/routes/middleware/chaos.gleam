import gleam/erlang/process
import gleam/int
import routes/error.{internal, send_error}
import wisp.{type Request, type Response}

pub fn chaos_middleware(_request: Request, next: fn() -> Response) -> Response {
  // Similar to node sleep is in ms not µs but also gleam only uses integers
  case int.random(101) < 5 {
    True -> process.sleep(int.random(2) + 1)
    False -> Nil
  }

  case int.random(101) < 5 {
    True -> send_error(internal)
    False -> next()
  }
}
