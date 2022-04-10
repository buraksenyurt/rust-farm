/*
   HTTP Request Body içerisinden gelen JSON İçeriğinin karşılığı olan veri yapısı.
   Çok doğal olarak Deserialize edilebilir olması gerekir.
*/
use crate::Size;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct WorkItem {
    pub title: String,
    pub size: Size,
}
