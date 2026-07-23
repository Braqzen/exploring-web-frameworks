pub type Operation {
  Transform
  Merge
  Sort
  Compute
}

pub fn operation_to_string(operation: Operation) -> String {
  case operation {
    Transform -> "Transform"
    Merge -> "Merge"
    Sort -> "Sort"
    Compute -> "Compute"
  }
}
