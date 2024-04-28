use axum::Form;
use hyper::StatusCode;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(Form(data): Form<FormData>) -> StatusCode {
    println!("name {} email {}", data.name, data.email);
    StatusCode::OK
}
