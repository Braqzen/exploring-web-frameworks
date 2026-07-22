import gleam/dynamic/decode
import gleam/string

pub type Operation {
  Transform
  Merge
  Sort
  Compute
}

pub fn decode_operation() -> decode.Decoder(Operation) {
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

pub fn encode_operation(operation: Operation) -> String {
  case operation {
    Transform -> "Transform"
    Merge -> "Merge"
    Sort -> "Sort"
    Compute -> "Compute"
  }
}
