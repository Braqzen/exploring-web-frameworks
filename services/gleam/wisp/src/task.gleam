import operation.{type Operation}

pub type Task {
  Task(secret: String, operation: Operation)
}

pub type PatchedTask {
  PatchedTask(operation: Operation)
}
