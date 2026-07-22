import palabres
import palabres/level
import palabres/options

// Note: Erlang has some module named logger therefore it is reserved and we must use logger_.gleam

pub fn configure_logger(min_level: level.Level) {
  options.defaults()
  |> options.json(True)
  |> options.level(min_level)
  |> options.output(to: options.stdout())
  |> palabres.configure
}
