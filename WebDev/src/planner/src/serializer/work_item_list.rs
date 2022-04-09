use crate::work_item::mission::Mission;
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

impl WorkItemList {
    /*
       WorkItemList'in yeni bir örneğini oluştururken kullanılan bu yapıcı metot,
       Misson türünden gelen vector içeriğini kullanarak geriye WorkItemList döndürmekte.
       Bu noktada new ile yapıcı metot olarak görev yapan bir fonksiyonun,
       farklı türden değer döndürebileceğini de öğrenmiş oldum.
    */
    pub fn new(missions: Vec<Mission>) -> WorkItemList {
        let mut ready_items_buf = vec![];
        let mut doing_items_buf = vec![];
        let mut completed_items_buf = vec![];

        // Parametre olarak gelen Mission nesneleri dolaşılır ve türe göre header değerleri
        // uygun vector listelerine eklenir.
        for mission in missions {
            match mission {
                Mission::Ready(item) => ready_items_buf.push(item.header),
                Mission::Doing(item) => doing_items_buf.push(item.header),
                Mission::Completed(item) => completed_items_buf.push(item.header),
            }
        }
        let (count1, count2, count3) = (
            ready_items_buf.len() as u8,
            doing_items_buf.len() as u8,
            completed_items_buf.len() as u8,
        );

        WorkItemList {
            ready_items: ready_items_buf,
            doing_items: doing_items_buf,
            completed_items: completed_items_buf,
            ready_items_count: count1,
            doing_items_count: count2,
            completed_items_count: count3,
        }
    }
}
