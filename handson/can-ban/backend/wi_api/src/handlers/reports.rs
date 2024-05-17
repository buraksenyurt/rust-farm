use crate::state::AppState;
use actix_web::web::Data;
use actix_web::{HttpResponse, Responder};
use log::error;

pub struct ReportHandler {}

impl ReportHandler {
    pub async fn get_board_summary_report(data: Data<AppState>) -> impl Responder {
        let db = data.db_context.lock().unwrap();
        match db.get_summary_report() {
            Ok(response) => HttpResponse::Ok().json(response),
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }
}
