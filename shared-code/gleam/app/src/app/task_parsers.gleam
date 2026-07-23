import app/internal/task_decoders.{patched_task_decoder, task_decoder}
import app/task.{type PatchedTask, type Task}
import gleam/json.{type DecodeError}

pub fn parse_task(body: BitArray) -> Result(Task, DecodeError) {
  json.parse_bits(body, task_decoder())
}

pub fn parse_patched_task(body: BitArray) -> Result(PatchedTask, DecodeError) {
  json.parse_bits(body, patched_task_decoder())
}
