use super::action::Action;
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

fn process_ready(wi: Ready, action: Action, state: &Map<String, Value>) {
    let mut state = state.clone();
    match action {
        Action::Get => {
            wi.get(&wi.header.title, &state);
        }
        Action::Create => wi.create(&wi.header.title, &wi.size, Status::Ready, &mut state),
        Action::Edit => wi.set_to_doing(&wi.header.title, &mut state),
        Action::Delete => wi.delete(&wi.header.title, &mut state),
    }
}

fn process_doing(wi: Doing, action: Action, state: &Map<String, Value>) {
    let mut state = state.clone();
    match action {
        Action::Get => {
            wi.get(&wi.header.title, &state);
        }
        Action::Edit => wi.set_to_complete(&wi.header.title, &mut state),
        Action::Delete => wi.delete(&wi.header.title, &mut state),
        _ => {}
    }
}

fn process_completed(wi: Completed, action: Action, state: &Map<String, Value>) {
    let mut state = state.clone();
    match action {
        Action::Get => {
            wi.get(&wi.header.title, &state);
        }
        Action::Edit => wi.set_to_doing(&wi.header.title, &mut state),
        Action::Delete => wi.delete(&wi.header.title, &mut state),
        _ => {}
    }
}

/// Bir görev için belirtilen parametrelerde aksiyon başlatır
pub fn run(mission: Mission, action: Action, state: &mut Map<String, Value>) {
    match mission {
        Mission::Ready(wi) => process_ready(wi, action, state),
        Mission::Doing(wi) => process_doing(wi, action, state),
        Mission::Completed(wi) => process_completed(wi, action, state),
    }
}
