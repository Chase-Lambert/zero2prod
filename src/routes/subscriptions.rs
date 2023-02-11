use actix_web::web::Form;
use actix_web::HttpResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(_form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
