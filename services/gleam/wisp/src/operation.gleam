import gleam/dynamic/decode

pub type Operation {
  Transform
  Merge
  Sort
  Compute
}

pub fn decode_operation() -> decode.Decoder(Operation) {
  use value <- decode.then(decode.string)

  // Shit design. Literally a placeholder value, use anything in the .failure(first arg)
  case value {
    "transform" -> decode.success(Transform)
    "merge" -> decode.success(Merge)
    "sort" -> decode.success(Sort)
    "compute" -> decode.success(Compute)
    _ -> decode.failure(Transform, "Operation")
  }
}

pub fn encode_operation(operation: Operation) -> String {
  case operation {
    Transform -> "transform"
    Merge -> "merge"
    Sort -> "sort"
    Compute -> "compute"
  }
}
