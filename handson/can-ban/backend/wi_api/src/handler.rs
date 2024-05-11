use crate::app_state::AppState;
use crate::model::{
    CreateWorkItemRequest, CreateWorkItemResponse, Status, UpdateStatusRequest, WorkItem,
};
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Responder};
use chrono::Local;
use log::info;

pub struct Handler {}

impl Handler {
    pub async fn create(
        item: web::Json<CreateWorkItemRequest>,
        data: Data<AppState>,
    ) -> impl Responder {
        let db = data.db_context.lock().unwrap();
        let payload = item.into_inner();
        let new_item = WorkItem {
            id: 0,
            title: payload.title,
            duration: payload.duration,
            duration_type: payload.duration_type,
            size: payload.size,
            status: Status::Todo,
            crate_date: Local::now(),
            modified_date: None,
        };

        match db.add_work_item(&new_item) {
            Ok(id) => {
                info!("{id}, New item has been added");
                let response = CreateWorkItemResponse {
                    id,
                    title: new_item.title,
                    duration: new_item.duration,
                    duration_type: new_item.duration_type,
                    size: new_item.size,
                    status: new_item.status,
                };
                HttpResponse::Created().json(response)
            }
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    pub async fn update_state(
        body: web::Json<UpdateStatusRequest>,
        data: Data<AppState>,
    ) -> impl Responder {
        let db = data.db_context.lock().unwrap();
        let payload = body.into_inner();
        info!("{:?}", payload);
        match db.update_work_item_status(&payload) {
            Ok(_) => HttpResponse::Accepted().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    pub async fn get(id: web::Path<u32>, data: Data<AppState>) -> impl Responder {
        let db = data.db_context.lock().unwrap();
        match db.get_item(*id) {
            Ok(response) => HttpResponse::Ok().json(response),
            Err(_) => HttpResponse::NotFound().finish(),
        }
    }

    pub async fn get_all(data: Data<AppState>) -> impl Responder {
        let db = data.db_context.lock().unwrap();
        match db.get_all() {
            Ok(result) => HttpResponse::Ok().json(result),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}
