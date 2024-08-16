
use actix_multipart::{form::MultipartFormConfig, MultipartError};

use actix_web::{error::Error, App, HttpRequest, HttpServer};

// use tokio::join;
use renderenko::post::render::render_config;

fn error_handler(err:MultipartError, _req: &HttpRequest) -> Error {
    // println!("Error: {:?}\n Request: {:?}",err,req);
    Error::from(err)
    // Error::from(RenderenkoError {name: "something"})
}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    // let config:MultipartFormConfig = MultipartFormConfig::default();
    // config.error_handler(error_handler);
    let ip = "127.0.0.1";
    let port = 8080;
    let server = HttpServer::new(|| {
        App::new()
            .configure(render_config)
            .app_data(MultipartFormConfig::default().error_handler(error_handler))
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind((ip,port))?;
    let task_handle = tokio::spawn(server.run());
    println!("server is listening on {}:{}",ip,port);
    let _ = task_handle.await;
    Ok(())
}