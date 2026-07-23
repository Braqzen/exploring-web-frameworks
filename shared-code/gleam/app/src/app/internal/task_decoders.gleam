import app/decoders.{operation_decoder}
import app/task.{type PatchedTask, type Task, PatchedTask, Task}
import gleam/dynamic/decode

pub fn task_decoder() -> decode.Decoder(Task) {
  use secret <- decode.field("secret", decode.string)
  use operation <- decode.field("operation", operation_decoder())
  decode.success(Task(secret, operation))
}

pub fn patched_task_decoder() -> decode.Decoder(PatchedTask) {
  use operation <- decode.field("operation", operation_decoder())
  decode.success(PatchedTask(operation))
}
