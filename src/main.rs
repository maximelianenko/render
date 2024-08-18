
use actix_multipart::{form::MultipartFormConfig, MultipartError};

use actix_web::{error::Error, App, HttpRequest, HttpServer};

use http_serde::http::StatusCode;
// use tokio::join;
use renderenko::post::render::{render_config, APIResponse};


fn error_handler(err:MultipartError, _req: &HttpRequest) -> Error {
    // println!("Error: {:?}\n Request: {:?}",err,req);
    let res = match err {
        MultipartError::NoContentDisposition => {
            APIResponse::new(
                StatusCode::BAD_REQUEST,
                "Content-Disposition",
                "wrong Content-Disposition header is set",
                ""
            )
        },
        MultipartError::NoContentType => {
            APIResponse::new(
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Content-Type",
                "wrong Content-Type header is set",
                ""
            )
        },
        MultipartError::ParseContentType => {
            APIResponse::new(
                StatusCode::BAD_REQUEST,
                "Content-Type",
                "couldn't parse Content-Type header",
                ""
            )
        },
        MultipartError::Boundary => {
            APIResponse::new(
                StatusCode::BAD_REQUEST,
                "multipart",
                "multipart boundary is not found",
                ""
            )
        },
        MultipartError::Nested => {
            APIResponse::new(
                StatusCode::BAD_REQUEST,
                "multipart",
                "nested multipart is not supported",
                ""
            )
        },
        MultipartError::Incomplete => {
            APIResponse::new(
                StatusCode::BAD_REQUEST,
                "multipart",
                "multipart stream is incomplete",
                ""
            )
        },
        MultipartError::Parse(_err) => {
            APIResponse::new(
                StatusCode::BAD_REQUEST,
                "",
                "error during field parsing",
                ""
            )
        },
        MultipartError::Payload(_err) => {
            APIResponse::new(
                StatusCode::BAD_REQUEST,
                "",
                "payload error",
                ""
            )
        },
        MultipartError::NotConsumed => {
            APIResponse::new(
                StatusCode::BAD_REQUEST,
                "",
                "not consumed",
                ""
            )
        },
        MultipartError::Field { field_name, source } => {
            APIResponse::new(
                StatusCode::BAD_REQUEST,
                &field_name,
                &format!("deserialize error in field '{}'", field_name),
                &source.to_string()
            )
        },
        MultipartError::DuplicateField(field_name) => {
            APIResponse::new(
                StatusCode::BAD_REQUEST,
                &field_name,
                &format!("dublicate field {}", field_name),
                ""
            )
        },
        MultipartError::MissingField(field_name) => {
            APIResponse::new(
                StatusCode::BAD_REQUEST,
                &field_name,
                &format!("missing required field '{}'", field_name),
                ""
            )
        },
        MultipartError::UnsupportedField(field_name) => {
            APIResponse::new(
                StatusCode::BAD_REQUEST,
                &field_name,
                &format!("unsupported field {}", field_name),
                ""
            )
        },
        _ => {
            APIResponse::new(
                StatusCode::IM_A_TEAPOT,
                "unknown",
                "unknown error",
                "idk"
            )
        },
    };
    
    return Error::from(res)
    // Error::from(APIResponse {
    //     field: err.field_name,
    //     message: String::from("field"),
    //     description: String::from("field")
    // })
    // Error::from(RenderenkoError {name: "something"})
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