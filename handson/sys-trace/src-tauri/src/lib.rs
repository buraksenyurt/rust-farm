mod engine;
mod metrics;

use std::sync::Mutex;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};

use crate::metrics::Metrics;

/*
    Uygulamamız çalışırken System ve Disk bilgilerinin sık sık güncellenmesi gerekecek ancak bu iş
    çapraz çağrılarda eş zamanlılık sorunlarına yol açabilir.
    Bu nedenle System ve Disk nesnelerini Mutex ile sarmalayarak eş zamanlı erişimi güvenli hale getiriyoruz.
    Mutex, birden fazla iş parçacığının aynı anda System ve Disk nesnelerine erişmesini engeller
    ve sadece bir iş parçacığına erişim izni verir.

    Command ile işaretlenmiş fonksiyonlar worker thread pool üzerinden çağırıldığı için de
    bu Mutex yapısı, System ve Disk nesnelerine erişimde güvenliği sağlar.
*/
pub struct AppState {
    pub system: Mutex<System>,
    pub disks: Mutex<Disks>,
}

impl AppState {
    fn new() -> Self {
        /*
            Sadece CPU ve bellek bilgilerini izleyeceğiz (track). Diğer sistem bilgilerine ihtiyacımız yok.
            Bu nedenle RefreshKind ile sadece gerekli alanları güncelleyecek şekilde bir yapılandırma yapıyoruz.
        */
        let refresh = RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything());

        Self {
            system: Mutex::new(System::new_with_specifics(refresh)),
            disks: Mutex::new(Disks::new_with_refreshed_list()),
        }
    }
}

#[tauri::command]
fn get_metrics(state: tauri::State<'_, AppState>) -> Result<Metrics, String> {
    let mut system = state
        .system
        .lock()
        .map_err(|_| " Sys state poisoned".to_string())?;
    let mut disks = state
        .disks
        .lock()
        .map_err(|_| " Disk state poisoned".to_string())?;
    Ok(engine::collect(&mut system, &mut disks))
}

#[tauri::command]
fn get_host_info(state: tauri::State<'_, AppState>) -> Result<metrics::HostInfo, String> {
    let system = state
        .system
        .lock()
        .map_err(|_| " Sys state poisoned".to_string())?;
    Ok(engine::get_host_info(&system))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![get_metrics, get_host_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
