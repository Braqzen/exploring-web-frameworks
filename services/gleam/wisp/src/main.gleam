import envoy
import gleam/result
import server.{new_server, run_server}

// TODO: SOCKET="0.0.0.0:8000" ERL_FLAGS="+B" gleam run -m main
// kill the output

pub fn main() -> Nil {
  let assert Ok(socket) =
    envoy.get("SOCKET")
    |> result.map_error(fn(_) { panic as "SOCKET not set" })

  let assert Ok(server) =
    new_server(socket)
    |> result.map_error(fn(error) { panic as error })

  run_server(server)
}
