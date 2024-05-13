use crate::app_state::AppState;
use crate::model::{
    CreateWorkItemRequest, MoveToArchiveRequest, Status, UpdateStatusRequest, WorkItem,
    WorkItemResponse,
};
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Responder};
use chrono::Local;
use log::{error, info};

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
                let response = WorkItemResponse {
                    id,
                    title: new_item.title,
                    duration: new_item.duration,
                    duration_type: new_item.duration_type,
                    size: new_item.size,
                    status: new_item.status,
                };
                HttpResponse::Created().json(response)
            }
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
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
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }

    pub async fn move_to_archive(
        body: web::Json<MoveToArchiveRequest>,
        data: Data<AppState>,
    ) -> impl Responder {
        let db = data.db_context.lock().unwrap();
        match db.move_to_archive(body.into_inner().id) {
            Ok(_) => HttpResponse::Accepted().finish(),
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }

    pub async fn get(id: web::Path<u32>, data: Data<AppState>) -> impl Responder {
        let db = data.db_context.lock().unwrap();
        match db.get_item(*id) {
            Ok(response) => HttpResponse::Ok().json(response),
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }

    pub async fn get_all(data: Data<AppState>) -> impl Responder {
        let db = data.db_context.lock().unwrap();
        match db.get_all() {
            Ok(result) => HttpResponse::Ok().json(result),
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }

    pub async fn get_count(data: Data<AppState>) -> impl Responder {
        let db = data.db_context.lock().unwrap();
        match db.get_count() {
            Ok(result) => {
                info!("Total items {}", result);
                HttpResponse::Ok().json(result)
            }
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }
}
