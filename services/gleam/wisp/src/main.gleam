import envoy
import gleam/result
import palabres/level
import server.{new_server, run_server}
import telemetry/telemetry.{configure_logger}

pub fn main() -> Nil {
  let assert Ok(log_level) =
    envoy.get("LOG_LEVEL")
    |> result.map_error(fn(_) { panic as "LOG_LEVEL env error" })

  case level.from_string(log_level) {
    Ok(min) -> configure_logger(min)
    Error(_) -> panic as "invalid LOG_LEVEL"
  }

  let assert Ok(socket) =
    envoy.get("SOCKET")
    |> result.map_error(fn(_) { panic as "SOCKET not set" })

  let assert Ok(server) =
    new_server(socket)
    |> result.map_error(fn(error) { panic as error })

  run_server(server)
}
