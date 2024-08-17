use std::{borrow::Borrow, fs::File, io::{BufReader, BufWriter}, path::Path, sync::Arc};

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{http::StatusCode, web, HttpResponse, Responder};

use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder, ImageFormat};
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
    skin_texture: TempFile,
    width: Option<Text<u32>>,
    height: Option<Text<u32>>,
    resize: Option<Text<bool>>,
    resize_width: Option<Text<u32>>,
    resize_height: Option<Text<u32>>,
    skin_type: Option<Text<String>>,
    fov: Option<Text<f32>>,
    far: Option<Text<f32>>,
    near: Option<Text<f32>>,

    cam_pos_x: Option<Text<f32>>,
    cam_pos_y: Option<Text<f32>>,
    cam_pos_z: Option<Text<f32>>,

    cam_rot_x: Option<Text<f32>>,
    cam_rot_y: Option<Text<f32>>,

    mod_pos_x: Option<Text<f32>>,
    mod_pos_y: Option<Text<f32>>,
    mod_pos_z: Option<Text<f32>>,
    
    mod_rot_x: Option<Text<f32>>,
    mod_rot_y: Option<Text<f32>>,

}

macro_rules! text_unwrap_or {
    ( $x:expr, $y:expr ) => {
        {
            let field;
            if $x.is_some() {
                field = $x.unwrap().into_inner()
            } else {
                field = $y
            };
            field
        }
    };
}
macro_rules! minmax {
    ( $min: expr, $max: expr, $value:expr) => {
            {
                let field;
                if $value > $min {
                    if $value < $max {
                        field = $value;
                    } else {
                        field = $max;
                    }
                } else {
                    field = $min;
                }
                field
            }
    }
}
macro_rules! minmax_text_unwrap_or {
    ( $min: expr, $max: expr, $x:expr, $y:expr) => {
        {
            let field;
            if $x.is_some() {
                let value = $x.unwrap().into_inner();
                field = minmax!($min,$max,value);
            } else {
                field = $y;
            };
            field
        }
    };
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
pub async fn render(MultipartForm(form): MultipartForm<UploadForm>,data: web::Data<AppState>) -> impl Responder {
    if form.skin_texture.content_type.is_none() || form.skin_texture.content_type.unwrap() != "image/png" {
        return HttpResponse::build(StatusCode::BAD_REQUEST).body("Only png images are supported");
    }
    
    let file = match File::open(form.skin_texture.file) {
        Err(error) => {
            return HttpResponse::build(StatusCode::BAD_REQUEST).body(format!("Something wrong with file. \n Error: {}", error));
        }
        Ok(value) => value
    };
    let texture = match image::load(BufReader::new(file), ImageFormat::Png) {
        Err(error) => {
            return HttpResponse::build(StatusCode::BAD_REQUEST).body(format!("Something wrong with file. \n Error: {}", error));
        }
        Ok(value) => value
    };

    let texture_aspect_ratio = texture.width() / texture.height();
    // println!("{}",texture_aspect_ratio);
    if texture_aspect_ratio != 1 && texture_aspect_ratio != 2 {
        return HttpResponse::build(StatusCode::BAD_REQUEST).body("Texture aspect ratio needs to be 1/1 or 2/1 (64x64, 256x256, 64x32 etc...)");
    }

    let width = minmax_text_unwrap_or!(10,1000,form.width,100);
    let height = minmax_text_unwrap_or!(10,1000,form.height,100);
    let resize = text_unwrap_or!(form.resize, false);
    let resize_width = minmax_text_unwrap_or!(10,1000,form.resize_width,500);
    let resize_height = minmax_text_unwrap_or!(10,1000,form.resize_height,500);

    let skin_type: Skin;
    if texture_aspect_ratio == 2 {
        skin_type = Skin::SteveOld
    } else {
        let skin_type_string = text_unwrap_or!(form.skin_type,String::from("alex")).to_lowercase();
        if &skin_type_string == "steve" {
            skin_type = Skin::Steve
        } else {
            skin_type = Skin::Alex
        }
    }

    let fov = minmax_text_unwrap_or!(10.0, 120.0, form.fov, 30.0);

    let far = minmax_text_unwrap_or!(0.0,1000.0,form.far, 1000.0);
    let near = minmax_text_unwrap_or!(0.0,1000.0,form.near, 0.1);

    let cam_pos_x = minmax_text_unwrap_or!(0.0,1000.0,form.cam_pos_x,0.0);
    let cam_pos_y = minmax_text_unwrap_or!(0.0,1000.0,form.cam_pos_y,0.0);
    let cam_pos_z = minmax_text_unwrap_or!(0.0,1000.0,form.cam_pos_z,0.0);

    let cam_rot_x = minmax_text_unwrap_or!(0.0,360.0,form.cam_rot_x,0.0);
    let cam_rot_y = minmax_text_unwrap_or!(0.0,360.0,form.cam_rot_y,0.0);

    let mod_pos_x = minmax_text_unwrap_or!(0.0,1000.0,form.mod_pos_x,0.0);
    let mod_pos_y = minmax_text_unwrap_or!(0.0,1000.0,form.mod_pos_y,0.25);
    let mod_pos_z = minmax_text_unwrap_or!(0.0,1000.0,form.mod_pos_z,1.25);

    let mod_rot_x = minmax_text_unwrap_or!(0.0,360.0,form.mod_rot_x,0.0);
    let mod_rot_y = minmax_text_unwrap_or!(0.0,360.0,form.mod_rot_y,-15.0);

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
        return HttpResponse::build(StatusCode::OK)
                .content_type("image/png")
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
        return HttpResponse::build(StatusCode::OK)
                .content_type("image/png")
                // .insert_header(("Access-Control-Allow-Origin", "*"))
                .body(result_buf.into_inner().unwrap())
    }
    
}
