import application_.{type Application, Application, router}
import gleam/erlang/process
import gleam/int
import gleam/result
import gleam/string
import mist
import state.{new_app_state}
import wisp
import wisp/wisp_mist

pub type Server {
  Server(socket: Socket, application: Application)
}

pub fn new_server(socket: String) -> Result(Server, String) {
  use socket <- result.try(parse_socket(socket))

  use state <- result.try(
    new_app_state()
    |> result.map_error(fn(_) { "failed to start state actor" }),
  )

  let application = Application(state)
  Ok(Server(socket, application))
}

pub fn run_server(server: Server) {
  let secret_key_base = wisp.random_string(64)

  let builder =
    router(server.application, _)
    |> wisp_mist.handler(secret_key_base)
    |> mist.new
    |> mist.bind(server.socket.host)
    |> mist.port(server.socket.port)

  // Note: there is no graceful shutdowns without something like FFI into erlang
  case mist.start(builder) {
    Ok(_) -> process.sleep_forever()
    Error(_) -> Nil
  }
}

pub type Socket {
  Socket(host: String, port: Int)
}

fn parse_socket(socket: String) -> Result(Socket, String) {
  case string.split_once(socket, on: ":") {
    Ok(#(host, port)) ->
      case int.parse(port) {
        Ok(port) -> Ok(Socket(host: host, port: port))
        Error(_) -> Error("invalid port")
      }
    Error(_) -> Error("invalid socket")
  }
}
