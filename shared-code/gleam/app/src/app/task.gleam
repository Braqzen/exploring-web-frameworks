import app/operation.{type Operation, decode_operation, encode_operation}
import gleam/dynamic/decode
import gleam/json.{type DecodeError}

pub type Task {
  Task(secret: String, operation: Operation)
}

pub type PatchedTask {
  PatchedTask(operation: Operation)
}

pub fn parse_task(body: BitArray) -> Result(Task, DecodeError) {
  json.parse_bits(body, task_decoder())
}

pub fn parse_patched_task(body: BitArray) -> Result(PatchedTask, DecodeError) {
  json.parse_bits(body, patched_task_decoder())
}

fn task_decoder() -> decode.Decoder(Task) {
  use secret <- decode.field("secret", decode.string)
  use operation <- decode.field("operation", decode_operation())
  decode.success(Task(secret, operation))
}

fn patched_task_decoder() -> decode.Decoder(PatchedTask) {
  use operation <- decode.field("operation", decode_operation())
  decode.success(PatchedTask(operation))
}

pub fn encode_task(task: Task) -> String {
  json.object([
    #("secret", json.string(task.secret)),
    #("operation", json.string(encode_operation(task.operation))),
  ])
  |> json.to_string
}
