import gleam/result
import server.{new_server, run_server}

// TODO: ERL_FLAGS="+B" gleam run -m main
// kill the output

pub fn main() -> Nil {
  new_server("0.0.0.0:8000")
  |> result.map(run_server)
  |> result.unwrap(or: Nil)
}
