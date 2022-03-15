use super::command::Command;
use super::work_item::model::completed::Completed;
use super::work_item::model::doing::Doing;
use super::work_item::model::ready::Ready;
use crate::work_item::mission::Mission;
use crate::work_item::model::traits::create::Create;
use crate::work_item::model::traits::delete::Delete;
use crate::work_item::model::traits::edit::Edit;
use crate::work_item::model::traits::get::Get;
use crate::work_item::status::Status;
use serde_json::{Map, Value};

fn process_ready(wi: Ready, command: Command, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command {
        Command::Get => {
            wi.get(&wi.header.title, &state);
        }
        Command::Create => wi.create(&wi.header.title, &wi.size, Status::Ready, &mut state),
        Command::Edit => wi.set_to_doing(&wi.header.title, &mut state),
        Command::Delete => wi.delete(&wi.header.title, &mut state),
    }
}

fn process_doing(wi: Doing, command: Command, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command {
        Command::Get => {
            wi.get(&wi.header.title, &state);
        }
        Command::Edit => wi.set_to_complete(&wi.header.title, &mut state),
        Command::Delete => wi.delete(&wi.header.title, &mut state),
        _ => {}
    }
}

fn process_completed(wi: Completed, command: Command, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command {
        Command::Get => {
            wi.get(&wi.header.title, &state);
        }
        Command::Edit => wi.set_to_doing(&wi.header.title, &mut state),
        Command::Delete => wi.delete(&wi.header.title, &mut state),
        _ => {}
    }
}

pub fn run(mission: Mission, command: Command, state: &mut Map<String, Value>) {
    match mission {
        Mission::Ready(wi) => process_ready(wi, command, state),
        Mission::Doing(wi) => process_doing(wi, command, state),
        Mission::Completed(wi) => process_completed(wi, command, state),
    }
}
