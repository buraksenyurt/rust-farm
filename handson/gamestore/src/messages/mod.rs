pub mod request;
pub mod response;

use crate::entity::{developer, game};
pub use request::*;
pub use response::*;

impl From<&developer::Model> for DeveloperResponse {
    fn from(source: &developer::Model) -> Self {
        Self {
            id: source.id,
            fullname: source.fullname.to_owned(),
            about: source.about.to_owned(),
            level: source.level,
        }
    }
}

impl From<&game::Model> for GameResponse {
    fn from(source: &game::Model) -> Self {
        Self {
            id: source.id,
            developer_id: source.developer_id,
            title: source.title.to_owned(),
            summary: source.summary.to_owned(),
            year: source.year.to_owned(),
        }
    }
}
