mod csv_ds;
mod json_ds;

pub use csv_ds::*;
pub use json_ds::*;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait JsonLoader {
    async fn load_data<T: DeserializeOwned>(&self) -> T;
}

pub trait TabularLoader {
    async fn load_data<T: Serialize + DeserializeOwned + Send>(&self) -> Vec<T>;
}
