use actix_web::{get, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    format!(" i am home, I am your boss, I Like you")
}
