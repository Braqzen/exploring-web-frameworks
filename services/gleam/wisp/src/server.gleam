import app/config.{FileRead, JsonDecode, MissingService, new_app_config}
import app/state.{new_app_state}
import application_.{type Application, Application, router}
import gleam/erlang/process
import gleam/int
import gleam/result
import gleam/string
import mist
import palabres
import wisp
import wisp/wisp_mist

pub opaque type Server {
  Server(socket: Socket, application: Application)
}

pub fn new_server(socket: String) -> Result(Server, String) {
  use socket <- result.try(
    parse_socket(socket)
    |> result.map_error(fn(socket_error) {
      case socket_error {
        Format -> "Invalid socket format"
        Port -> "Invalid socket port"
      }
    }),
  )

  use config <- result.try(
    new_app_config()
    |> result.map_error(fn(error) {
      case error {
        MissingService -> "SERVICE env error"
        FileRead(_) -> "failed to read provider.json"
        JsonDecode(_) -> "failed to decode provider.json"
      }
    }),
  )

  use state <- result.try(
    new_app_state(config)
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

  palabres.info("Starting router")
  |> palabres.string(
    "socket",
    string.join(
      [server.socket.host, int.to_string(server.socket.port)],
      with: ":",
    ),
  )
  |> palabres.log

  // Note: there is no graceful shutdowns without something like FFI into erlang
  case mist.start(builder) {
    Ok(_) -> process.sleep_forever()
    Error(_) -> panic as "Failed to start server"
  }
}

type Socket {
  Socket(host: String, port: Int)
}

fn parse_socket(socket: String) -> Result(Socket, SocketError) {
  case string.split_once(socket, on: ":") {
    Ok(#(host, port)) ->
      // Note: No checks on valididty of host
      case int.parse(port) {
        Ok(port) -> Ok(Socket(host: host, port: port))
        Error(_) -> Error(Port)
      }
    Error(_) -> Error(Format)
  }
}

type SocketError {
  Format
  Port
}
