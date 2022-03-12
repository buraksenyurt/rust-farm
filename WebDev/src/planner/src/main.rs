use log::info;
use work_item::factory::Factory;
use work_item::mission::Mission;
use work_item::model::traits::create::Create;
use work_item::model::traits::get::Get;
use work_item::status::Status;

mod state_manager;
mod work_item;

fn main() {
    env_logger::init();

    let cleaning = Factory::create_work_item(Status::Ready, "Odayı temizle", 8);
    match cleaning {
        Some(Mission::Ready(m)) => m.create(&m.header.title, m.header.value),
        Some(Mission::Doing(m)) => m.get(&m.header.title),
        Some(Mission::Completed(m)) => info!("{} tamalanmış", &m.header.title),
        _ => {}
    }
}
