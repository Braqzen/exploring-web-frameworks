import gleam/dict.{type Dict}
import gleam/erlang/process.{type Subject}
import gleam/otp/actor
import task.{type Task}
import youid/uuid.{type Uuid}

pub type Message {
  Insert(id: Uuid, task: Task, reply: Subject(Nil))
  Get(id: Uuid, reply: Subject(Result(Task, Nil)))
  Delete(id: Uuid, reply: Subject(Result(Nil, Nil)))
}

pub fn process_message(
  tasks: Dict(Uuid, Task),
  message: Message,
) -> actor.Next(Dict(Uuid, Task), Message) {
  case message {
    Insert(id, task, reply) -> {
      let tasks = dict.insert(tasks, id, task)
      process.send(reply, Nil)
      actor.continue(tasks)
    }
    Get(id, reply) -> {
      process.send(reply, dict.get(tasks, id))
      actor.continue(tasks)
    }
    Delete(id, reply) -> {
      case dict.has_key(tasks, id) {
        True -> {
          process.send(reply, Ok(Nil))
          actor.continue(dict.delete(tasks, id))
        }
        False -> {
          process.send(reply, Error(Nil))
          actor.continue(tasks)
        }
      }
    }
  }
}
