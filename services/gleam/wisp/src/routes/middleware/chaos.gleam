import gleam/erlang/process
import gleam/int
import routes/error.{internal, send_error}
import state.{type AppState}
import wisp.{type Request, type Response}

pub fn chaos_middleware(
  _request: Request,
  state: AppState,
  next: fn() -> Response,
) -> Response {
  let latency_enabled = state.config.latency.enabled
  let error_enabled = state.config.error.enabled
  let latency_rate = state.config.latency.rate
  let error_rate = state.config.error.rate

  // Similar to node sleep is in ms not µs but also gleam only uses integers
  case latency_enabled && int.random(101) < latency_rate {
    True -> process.sleep(int.random(2) + 1)
    False -> Nil
  }

  case error_enabled && int.random(101) < error_rate {
    True -> send_error(internal)
    False -> next()
  }
}
