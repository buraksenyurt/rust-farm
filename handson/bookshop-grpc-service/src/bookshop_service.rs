use crate::bookshop::bookshop_server::Bookshop;
use crate::bookshop::{GetSuggestionRequest, GetSuggestionResponse};
use crate::store::{load_books, Book};
use rand::{thread_rng, Rng};
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

        if let Some(books) = load_books() {
            let founded: Vec<Book> = books
                .into_iter()
                .filter(|b| b.category == request.get_ref().category)
                .collect();

            if !founded.is_empty() {
                let mut rng = thread_rng();
                let idx = rng.gen_range(0..founded.len());
                let b: Book = founded[idx].clone();
                let suggestion = GetSuggestionResponse {
                    book_id: b.book_id,
                    title: b.title,
                    author: b.authors,
                    category: b.category,
                    unit_price: b.unit_price,
                };
                return Ok(Response::new(suggestion));
            }
        }

        Ok(Response::new(GetSuggestionResponse {
            book_id: 0,
            title: "Not found".to_string(),
            author: "Not found".to_string(),
            category: "Not found".to_string(),
            unit_price: 0.0,
        }))
    }
}
