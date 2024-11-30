pub mod json_source;

pub use json_source::*;

use serde::de::DeserializeOwned;

pub trait DataSource {
    async fn load_data<T: DeserializeOwned>(&self) -> T;
}

pub async fn load_data<T, S>(data_source: S) -> T
where
    T: DeserializeOwned,
    S: DataSource,
{
    data_source.load_data().await
}
