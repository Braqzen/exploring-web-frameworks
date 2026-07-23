import app/operation.{operation_to_string}
import app/task.{type Task}
import gleam/http
import gleam/string
import palabres
import wisp.{type Request}
import youid/uuid.{type Uuid}

pub fn log_removed(id: Uuid, request: Request, task: Task) {
  palabres.info("Removed task")
  |> palabres.string("id", uuid.to_string(id))
  |> palabres.string("operation", operation_to_string(task.operation))
  |> palabres.string("method", http.method_to_string(request.method))
  |> palabres.int("secret", string.length(task.secret))
  |> palabres.log
}

pub fn log_not_found(id: Uuid, request: Request) {
  palabres.warning("Task not found")
  |> palabres.string("id", uuid.to_string(id))
  |> palabres.string("method", http.method_to_string(request.method))
  |> palabres.log
}

pub fn log_patched(id: Uuid, request: Request, task: Task, new_task: Task) {
  palabres.info("Patched task")
  |> palabres.string("id", uuid.to_string(id))
  |> palabres.string("from_operation", operation_to_string(task.operation))
  |> palabres.string("to_operation", operation_to_string(new_task.operation))
  |> palabres.string("method", http.method_to_string(request.method))
  |> palabres.int("secret", string.length(task.secret))
  |> palabres.log
}

pub fn log_retrieved(id: Uuid, request: Request, task: Task) {
  palabres.info("Retrieved task")
  |> palabres.string("id", uuid.to_string(id))
  |> palabres.string("operation", operation_to_string(task.operation))
  |> palabres.string("method", http.method_to_string(request.method))
  |> palabres.int("secret", string.length(task.secret))
  |> palabres.log
}

pub fn log_overwrote(id: Uuid, request: Request, task: Task, new_task: Task) {
  palabres.info("Overwrote task")
  |> palabres.string("id", uuid.to_string(id))
  |> palabres.string("from_operation", operation_to_string(task.operation))
  |> palabres.string("to_operation", operation_to_string(new_task.operation))
  |> palabres.string("method", http.method_to_string(request.method))
  |> palabres.int("from_secret", string.length(task.secret))
  |> palabres.int("to_secret", string.length(new_task.secret))
  |> palabres.log
}

pub fn log_inserted(id: Uuid, request: Request, task: Task) {
  palabres.info("Inserted new task")
  |> palabres.string("id", uuid.to_string(id))
  |> palabres.string("operation", operation_to_string(task.operation))
  |> palabres.string("method", http.method_to_string(request.method))
  |> palabres.int("secret", string.length(task.secret))
  |> palabres.log
}
