use log::info;

pub async fn logout() -> String {
    info!("Logout view");
    "Çıkış sayfası".to_string()
}
