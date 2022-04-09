use crate::work_item::model::base::Base;
use serde::Serialize;

/*
   İstemciye get talebi sonrası dönülecek içeriğin özel olarak serileştirilmesini istiyoruz.
   Bu nedenle Work Item'ların ayrı ayrı listesini ve toplam sayılarını tutan bir veri yapısı tanımlandı.
   Derivable Serialze direktifi ile içeriğin JSON formatında serileştirileceğini belirtmiş oluyoruz.
*/
#[derive(Serialize)]
pub struct WorkItemList {
    pub ready_items: Vec<Base>,
    pub doing_items: Vec<Base>,
    pub completed_items: Vec<Base>,
    pub ready_items_count: u8,
    pub doing_items_count: u8,
    pub completed_items_count: u8,
}
