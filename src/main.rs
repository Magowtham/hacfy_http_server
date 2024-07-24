use dotenv;
use actix_web::{web,App,HttpResponse,HttpServer};


#[actix_web::main]
async fn main()->std::io::Result<()>{
    dotenv::dotenv().ok();

    let server_port=dotenv::var("PORT").expect("failed to load the PORT env variable");

    HttpServer::new(|| App::new())
    .bind(("127.0.0.1",server_port))?
    .run()
    .await

}