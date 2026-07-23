import app/operation.{type Operation, operation_to_string}
import gleam/json

pub type Task {
  Task(secret: String, operation: Operation)
}

pub type PatchedTask {
  PatchedTask(operation: Operation)
}

pub fn task_to_json(task: Task) -> String {
  json.object([
    #("secret", json.string(task.secret)),
    #("operation", json.string(operation_to_string(task.operation))),
  ])
  |> json.to_string
}
