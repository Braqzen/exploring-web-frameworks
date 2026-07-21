import gleam/dict.{type Dict}
import gleam/erlang/process.{type Subject}
import gleam/otp/actor
import task.{type Task}
import youid/uuid.{type Uuid}

pub type Message {
  Insert(id: Uuid, task: Task, reply: Subject(Nil))
  Get(id: Uuid, reply: Subject(Result(Task, Nil)))
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
  }
}
