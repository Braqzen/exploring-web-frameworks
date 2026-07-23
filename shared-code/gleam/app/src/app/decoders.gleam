import app/operation.{type Operation, Compute, Merge, Sort, Transform}
import gleam/dynamic/decode
import gleam/string

pub fn operation_decoder() -> decode.Decoder(Operation) {
  use value <- decode.then(decode.string)

  case string.lowercase(value) {
    "transform" -> decode.success(Transform)
    "merge" -> decode.success(Merge)
    "sort" -> decode.success(Sort)
    "compute" -> decode.success(Compute)
    // Shit design. Literally a placeholder value, use anything in place of Transform
    _ -> decode.failure(Transform, "Operation")
  }
}
