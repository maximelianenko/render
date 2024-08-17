
use actix_multipart::{form::MultipartFormConfig, MultipartError};

use actix_web::{error::Error, App, HttpRequest, HttpServer};

// use tokio::join;
use renderenko::post::render::render_config;
// use http_serde::http::StatusCode as SerdeStatusCode;

fn error_handler(err:MultipartError, _req: &HttpRequest) -> Error {
    Error::from(err)
}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    // let config:MultipartFormConfig = MultipartFormConfig::default();
    // config.error_handler(error_handler);
    let ip = "0.0.0.0";
    let port = 3000;
    let server = HttpServer::new(|| {
        App::new()
            .configure(render_config)
            .app_data(MultipartFormConfig::default().error_handler(error_handler))
            // .route("/", web::get().to(|| HttpResponse::build(StatusCode::OK)))
    })
    .bind((ip,port))?;
    let task_handle = tokio::spawn(server.run());
    println!("server is listening on {}:{}",ip,port);
    let _ = task_handle.await;
    Ok(())
}