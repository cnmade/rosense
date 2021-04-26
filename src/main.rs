pub mod modules;
use actix_web::{get, web, App, HttpServer, Responder};

use modules::*;

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home::controllers::home_controller::index)
            .service(home::controllers::about_controller::me)
            .service(home::controllers::admin_controller::login)
            .service(home::controllers::admin_controller::logout)
            .service(home::controllers::payment_controller::index)
            .service(index)
    })
    .bind("127.0.0.1:8084")?
    .run()
    .await
}
