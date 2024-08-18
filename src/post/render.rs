use core::fmt;
use std::{borrow::Borrow, fs::File, io::{BufReader, BufWriter}, path::Path, sync::Arc};

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{http::StatusCode as ActixStatusCode, web, HttpResponse, Responder, ResponseError};

// use axum::{http::header, response::AppendHeaders};
use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder, ImageFormat};
use serde::Serialize;
use http_serde::http::StatusCode;
use crate::rend::{builder, misc::mesh::load_from_obj, types::MeshV4Plus};
pub struct AppState {
    // ferris: Renderenko,
    alex: Arc<MeshV4Plus>,
    steve: Arc<MeshV4Plus>,
    steve_old: Arc<MeshV4Plus>
}
pub enum Skin {
    Alex,
    Steve,
    SteveOld
}
#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "1MB")]
    texture: TempFile,
    width: Text<u32>,
    height: Text<u32>,
    resize: Text<bool>,
    resize_width: Text<u32>,
    resize_height: Text<u32>,
    skin_type: Text<String>,
    fov: Text<f32>,
    far: Text<f32>,
    near: Text<f32>,

    cam_pos_x: Text<f32>,
    cam_pos_y: Text<f32>,
    cam_pos_z: Text<f32>,

    cam_rot_x: Text<f32>,
    cam_rot_y: Text<f32>,

    mod_pos_x: Text<f32>,
    mod_pos_y: Text<f32>,
    mod_pos_z: Text<f32>,
    
    mod_rot_x: Text<f32>,
    mod_rot_y: Text<f32>,

}
// fn config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/app")
//             .route(web::get().to(|| HttpResponse::Ok().body("app")))
//             .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
//     )
//     .service(web::scope("/api").configure(scoped_config))
//     .route("/", web::get().to(|| HttpResponse::Ok().body("/")));
// }
pub fn render_config(cfg: &mut web::ServiceConfig) {
    cfg
        .app_data(web::Data::new(AppState {
            // ferris: builder::Renderenko::new().mesh(Path::new("data/model/ferris.obj")).unwrap().to_owned(),
            alex: Arc::new(load_from_obj(Path::new("data/model/default/alex.obj")).unwrap()),
            steve: Arc::new(load_from_obj(Path::new("data/model/default/steve.obj")).unwrap()),
            steve_old: Arc::new(load_from_obj(Path::new("data/model/default/steve_old.obj")).unwrap())
        }))
        .service(
            web::resource("/v1")
                .route(web::post().to(render))
        );
}

// macro_rules! block_or_response {
//     ($field: expr,$value:expr, $block:block) => {
//         if ($value.is_some()) {
//             $block
//         } else {
//             return APIResponse::new(
//                 StatusCode::BAD_REQUEST,
//                 $field,
//                 &format!("missing '{}' field", $field),
//                 ""
//             ).response_json()
//         }
//     };
// }

// '`"временное решение"`'
macro_rules! var_minmax_or_response {
    ($field:ident, $value:expr, $min:expr, $max:expr) => {
        let $field;
        minmax_or_response!($field,$value,$min,$max);
    };
}
macro_rules! minmax_or_response {
    ($field:ident, $value:expr, $min:expr, $max:expr) => {
        let value = $value.into_inner();
        if value >= $min && value <= $max {
            $field = value
        } else {
            return APIResponse::new(
                StatusCode::BAD_REQUEST,
                stringify!($field),
                &format!("field '{}' is out of limits", stringify!($field)),
                &format!("field '{}' value needs to be in range {}-{}",stringify!($field),$min,$max)
            ).response_json()
        }
    };
}
#[derive(Serialize, Debug, Clone)]
pub struct APIResponse {
    #[serde(with = "http_serde::status_code")]
    pub code: StatusCode,
    pub field: String,
    pub message: String,
    pub description: String
}

impl APIResponse {
    pub fn new(code: StatusCode, field: &str, message: &str, description: &str) -> APIResponse {
        APIResponse {
            code,
            field: field.to_string(),
            message: message.to_string(),
            description: description.to_string()
        }
    }
    pub fn response_json(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .append_header(("Access-Control-Allow-Origin","*"))
            .append_header(("Access-Control-Allow-Methods", "POST"))
            .json(&self)
    }
}

impl fmt::Display for APIResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}
impl ResponseError for APIResponse {
    fn status_code(&self) -> ActixStatusCode {
        ActixStatusCode::from_u16(self.code.into()).unwrap_or(ActixStatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        return self.response_json()
    }
}
pub async fn render(MultipartForm(form): MultipartForm<UploadForm>,data: web::Data<AppState>) -> impl Responder {
    if form.texture.content_type.is_none() || form.texture.content_type.unwrap() != "image/png" {
        return APIResponse::new(
                StatusCode::BAD_REQUEST,
            "texture",
            "texture must be in png format",
            "texture must be an image in png format and have size up to ~1MB"
        ).response_json()
    }
    
    let file = match File::open(form.texture.file) {
        Err(_error) => {
            return APIResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "texture",
                "texture file couldn't be opened on the server",
                "maybe texture file is corrupted"
            ).response_json()
        }
        Ok(value) => value
    };
    let texture = match image::load(BufReader::new(file), ImageFormat::Png) {
        Err(_error) => {
            return APIResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "texture",
                "texture file couldn't be readed on the server",
                "maybe texture file is corrupted"
            ).response_json()
        }
        Ok(value) => value
    };

    let texture_aspect_ratio = texture.width() / texture.height();
    // println!("{}",texture_aspect_ratio);

    if texture_aspect_ratio != 1 && texture_aspect_ratio != 2 {
        return APIResponse::new(
            StatusCode::BAD_REQUEST,
            "texture",
            "texture have wrong aspect ratio",
            "texture aspect-ratio needs to be 1/1 or 2/1 (64x64, 256x256, 64x32 etc...)"
        ).response_json()
    }

    var_minmax_or_response!(width,form.width, 10, 1000);
    var_minmax_or_response!(height,form.height, 10, 1000);
    // let width = minmax_text_unwrap_or!(10,1000,form.width,100);
    // let resize = text_unwrap_or!(form.resize, false);
    let resize = form.resize.into_inner();
    // minmax_or_response!(resize_width,form.resize_width, 10, 1000);
    // minmax_or_response!(resize_height,form.resize_height, 10, 1000);
    // let resize_width = minmax_text_unwrap_or!(10,1000,form.resize_width,500);
    // let resize_height = minmax_text_unwrap_or!(10,1000,form.resize_height,500);

    let skin_type_string = form.skin_type.into_inner();

    let skin_type: Skin = match skin_type_string.as_str() {
            "steve" => {
                Skin::Steve
            }
            "steve_old" => {
                Skin::SteveOld
            }
            _ => {
                Skin::Alex
            }
    };
    // if texture_aspect_ratio == 2 {
    //     skin_type = Skin::SteveOld
    // } else {
    //     let skin_type_string = text_unwrap_or!(form.skin_type,String::from("alex")).to_lowercase();
    //     if &skin_type_string == "steve" {
    //         skin_type = Skin::Steve
    //     } else {
    //         skin_type = Skin::Alex
    //     }
    // }

    // let fov = minmax_text_unwrap_or!(1.0, 120.0, form.fov, 30.0);
    var_minmax_or_response!(fov,form.fov, 1.0, 360.0);
    var_minmax_or_response!(far,form.far, 1.0, 1000.0);
    var_minmax_or_response!(near,form.near, 0.1, 1000.0);
    // let far = minmax_text_unwrap_or!(1.0,1000.0,form.far, 1000.0);
    // let near = minmax_text_unwrap_or!(0.1,1000.0,form.near, 0.1);
    var_minmax_or_response!(cam_pos_x,form.cam_pos_x, -1000.0, 1000.0);
    var_minmax_or_response!(cam_pos_y,form.cam_pos_y, -1000.0, 1000.0);
    var_minmax_or_response!(cam_pos_z,form.cam_pos_z, -1000.0, 1000.0);
    // let cam_pos_x = minmax_text_unwrap_or!(-1000.0,1000.0,form.cam_pos_x,0.0);
    // let cam_pos_y = minmax_text_unwrap_or!(-1000.0,1000.0,form.cam_pos_y,0.0);
    // let cam_pos_z = minmax_text_unwrap_or!(-1000.0,1000.0,form.cam_pos_z,0.0);
    var_minmax_or_response!(cam_rot_x,form.cam_rot_x, -360.0, 360.0);
    var_minmax_or_response!(cam_rot_y,form.cam_rot_y, -360.0, 360.0);
    // let cam_rot_x = minmax_text_unwrap_or!(-360.0,360.0,form.cam_rot_x,0.0);
    // let cam_rot_y = minmax_text_unwrap_or!(-360.0,360.0,form.cam_rot_y,0.0);

    var_minmax_or_response!(mod_pos_x,form.mod_pos_x, -1000.0, 1000.0);
    var_minmax_or_response!(mod_pos_y,form.mod_pos_y, -1000.0, 1000.0);
    var_minmax_or_response!(mod_pos_z,form.mod_pos_z, -1000.0, 1000.0);
    
    // let mod_pos_x = minmax_text_unwrap_or!(-1000.0,1000.0,form.mod_pos_x,0.0);
    // let mod_pos_y = minmax_text_unwrap_or!(-1000.0,1000.0,form.mod_pos_y,0.25);
    // let mod_pos_z = minmax_text_unwrap_or!(-1000.0,1000.0,form.mod_pos_z,1.25);
    var_minmax_or_response!(mod_rot_x,form.mod_rot_x, -360.0, 360.0);
    var_minmax_or_response!(mod_rot_y,form.mod_rot_y, -360.0, 360.0);
    // let mod_rot_x = minmax_text_unwrap_or!(-360.0,360.0,form.mod_rot_x,0.0);
    // let mod_rot_y = minmax_text_unwrap_or!(-360.0,360.0,form.mod_rot_y,-15.0);

    let mesh = match skin_type {
        Skin::Alex => Arc::clone(&data.alex),
        Skin::Steve => Arc::clone(&data.steve),
        Skin::SteveOld => Arc::clone(&data.steve_old)
    };
    let rendered = builder::Renderenko::new()
        .mesh_load(mesh)
        .texture_from_dynamicimage(texture).unwrap()
        .size(width, height)
        .camera(cam_pos_x,cam_pos_y,cam_pos_z,cam_rot_x,cam_rot_y)
        .fov(fov)
        .range(far, near)
        .world(mod_rot_x,mod_rot_y,mod_pos_x,mod_pos_y,mod_pos_z)
        .build()
        .render();
    if resize {
        var_minmax_or_response!(resize_width,form.resize_width, 10, 1000);
        var_minmax_or_response!(resize_height,form.resize_height, 10, 1000);
        let resized = rendered
        .resize(resize_width,resize_height);
        let image = resized.result();
        let mut result_buf = BufWriter::new(Vec::new());
        PngEncoder::new(&mut result_buf)
            .write_image(
                image.buffer(),
                image.width(),
                image.height(),
                ExtendedColorType::Rgba8,
            )
            .unwrap();
        return HttpResponse::build(ActixStatusCode::OK)
                .content_type("image/png")
                .append_header(("Access-Control-Allow-Origin","*"))
                .append_header(("Access-Control-Allow-Methods", "POST"))
                .body(result_buf.into_inner().unwrap())
    } else {
        let image = rendered.borrow().result();
        let mut result_buf = BufWriter::new(Vec::new());
        PngEncoder::new(&mut result_buf)
            .write_image(
                &image.to_rgba8(),
                image.width(),
                image.height(),
                ExtendedColorType::Rgba8,
            )
            .unwrap();
        return HttpResponse::build(ActixStatusCode::OK)
                .content_type("image/png")
                .append_header(("Access-Control-Allow-Origin","*"))
                .append_header(("Access-Control-Allow-Methods", "POST"))
                // .insert_header(("Access-Control-Allow-Origin", "*"))
                .body(result_buf.into_inner().unwrap())
    }
    
}
