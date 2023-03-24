use crate::bookshop::bookshop_server::Bookshop;
use crate::bookshop::{GetSuggestionRequest, GetSuggestionResponse};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct BookShopService {}

#[tonic::async_trait]
impl Bookshop for BookShopService {
    async fn get_suggestion(
        &self,
        request: Request<GetSuggestionRequest>,
    ) -> Result<Response<GetSuggestionResponse>, Status> {
        println!("Gelen talep {:?}", request.remote_addr());

        let suggestion = GetSuggestionResponse {
            book_id: 1,
            title: "Sistem".to_string(),
            author: "James Ball".to_string(),
            category: request.into_inner().category,
            unit_price: 34.50,
        };

        Ok(Response::new(suggestion))
    }
}
