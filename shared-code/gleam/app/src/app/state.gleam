import app/config.{type AppConfig}
import app/message.{type Message, process_message}
import app/task.{type Task}
import gleam/dict
import gleam/erlang/process.{type Subject}
import gleam/otp/actor
import youid/uuid.{type Uuid}

pub type AppState {
  AppState(tasks: Subject(Message), config: AppConfig)
}

pub type StateError {
  Timeout
  NotFound
}

pub fn new_app_state(config: AppConfig) -> Result(AppState, actor.StartError) {
  case
    actor.new(dict.new())
    |> actor.on_message(process_message)
    |> actor.start
  {
    Ok(started) -> Ok(AppState(tasks: started.data, config:))
    Error(err) -> Error(err)
  }
}

pub fn insert(state: AppState, id: Uuid, task: Task) -> Result(Nil, StateError) {
  let reply = process.new_subject()
  process.send(state.tasks, message.Insert(id, task, reply))

  case process.receive(reply, 1000) {
    Ok(Nil) -> Ok(Nil)
    Error(_) -> Error(Timeout)
  }
}

pub fn get(state: AppState, id: Uuid) -> Result(Task, StateError) {
  let reply = process.new_subject()
  process.send(state.tasks, message.Get(id, reply))

  case process.receive(reply, 1000) {
    Ok(Ok(task)) -> Ok(task)
    Ok(Error(Nil)) -> Error(NotFound)
    Error(_) -> Error(Timeout)
  }
}

pub fn delete(state: AppState, id: Uuid) -> Result(Task, StateError) {
  let reply = process.new_subject()
  process.send(state.tasks, message.Delete(id, reply))

  case process.receive(reply, 1000) {
    Ok(Ok(task)) -> Ok(task)
    Ok(Error(Nil)) -> Error(NotFound)
    Error(_) -> Error(Timeout)
  }
}
