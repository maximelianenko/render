use core::fmt;
use std::{path::Path, sync::Arc};

use fast_image_resize::{images::Image, IntoImageView, ResizeAlg, ResizeOptions, Resizer};
use image::{ImageReader, save_buffer, DynamicImage, ExtendedColorType, ImageResult};
use nalgebra::{Matrix4, Vector4};
use uuid::Uuid;


use super::{render, misc::{matrix::{matrix_projection, matrix_view, matrix_world}, mesh::load_from_obj}, types::MeshV4Plus};

macro_rules! some_if_none {
    ( $x:expr, $y:expr ) => {
        {
            if $x.is_none() {
                $x = Some($y)
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct LoadError;

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "load failed")
    }
}
// #[derive(Debug, Clone)]
pub struct RenderenkoBuilded {
    pub mesh: Arc<MeshV4Plus>,
    pub texture: DynamicImage,
    pub matrix_view: (Vector4<f32>,Matrix4<f32>),
    pub matrix_projection: Matrix4<f32>,
    pub matrix_world: Matrix4<f32>,
    pub width: f32,
    pub height: f32,
}

// #[derive(Debug, Clone)]
pub struct Renderenko {
    mesh: Option<Arc<MeshV4Plus>>,
    texture: Option<DynamicImage>,
    matrix_view: Option<(Vector4<f32>,Matrix4<f32>)>,
    matrix_world: Option<Matrix4<f32>>,
    width: Option<f32>,
    height: Option<f32>,
    fov: Option<f32>,
    far:Option<f32>,
    near:Option<f32>,
}
impl Renderenko {
    pub fn new() -> Renderenko {
        return Renderenko {
            mesh: None,
            texture: None,
            matrix_view:None,
            matrix_world: None,
            width: None,
            height: None,
            fov: None,
            far: None,
            near: None,
        }
    }
    pub fn mesh(&mut self,path: &Path) -> Result<&mut Self, LoadError> {
        let mesh = load_from_obj(path);
        if mesh.is_err() {
            return Err(LoadError);
        }
        self.mesh = Some(Arc::new(mesh.unwrap()));
        return Ok(self);
    }
    pub fn mesh_load(&mut self, mesh: Arc<MeshV4Plus>) -> &mut Self {
        self.mesh = Some(mesh);
        return self;
    }
    pub fn texture(&mut self,path: &Path) -> Result<&mut Self, LoadError> {
        let reader = ImageReader::open(path);
        if reader.is_err() {
            return Err(LoadError);
        }
        let reader_decode = reader.unwrap().decode();
        if reader_decode.is_err() {
            return Err(LoadError);
        }
        let texture = reader_decode.unwrap();
        self.texture = Some(texture);
        return Ok(self);
    }
    pub fn texture_from_dynamicimage(&mut self, image: DynamicImage) -> Result<&mut Self, LoadError> {
        self.texture = Some(image);
        return Ok(self);
    }
    pub fn size(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = Some(width as f32);
        self.height = Some(height as f32);
        return self;
    }
    pub fn fov(&mut self, fov: f32) -> &mut Self {
        self.fov = Some(fov);
        return self;
    }
    // мб сменить название
    pub fn range(&mut self,far:f32, near:f32) -> &mut Self {
        self.far = Some(far);
        self.near = Some(near); 
        return self;
    }
    pub fn world(&mut self,xr:f32,yr:f32,xt:f32,yt:f32,zt:f32) -> &mut Self {
        self.matrix_world = Some(matrix_world(xr, yr, xt, yt, zt));
        return self;
    }
    pub fn camera(&mut self, x: f32, y:f32, z:f32, pitch: f32, yaw:f32) -> &mut Self {
        self.matrix_view = Some(matrix_view(
            Vector4::new(x,y,z,1.0),
            Vector4::new(0.0,3.0,0.0,1.0),
            Vector4::new(0.0,0.0,1.0,1.0),
            pitch,
            yaw
        ));
        return self;
    }
    pub fn build(&mut self) -> RenderenkoBuilded {
        some_if_none!(self.width, 100.0);
        some_if_none!(self.height, 100.0);
        some_if_none!(self.near, 0.1);
        some_if_none!(self.far, 1000.0);
        some_if_none!(self.fov, 30.0);
        some_if_none!(self.matrix_world, matrix_world(
            0.0,
            -15.0,
            0.0,
            0.25,
            1.25
        ));
        some_if_none!(self.matrix_view, matrix_view(
            Vector4::new(0.0,0.0,0.0,1.0),
            Vector4::new(0.0,3.0,0.0,1.0),
            Vector4::new(0.0,0.0,1.0,1.0),
            0.0,
            0.0
        ));
        if self.texture.is_none() {
            panic!("provide texture")
        }
        if self.mesh.is_none() {
            panic!("provide mesh")
        }
        let aspect_ratio = self.height.unwrap() / self.width.unwrap();
        let matrix_projection = matrix_projection(
            self.fov.unwrap(), 
            aspect_ratio,
            self.near.unwrap(),
            self.far.unwrap()
        );
        return RenderenkoBuilded {
            width: self.width.unwrap(),
            height: self.height.unwrap(),
            matrix_projection,
            matrix_view: self.matrix_view.unwrap(),
            matrix_world: self.matrix_world.unwrap(),
            mesh: Arc::clone(self.mesh.as_ref().unwrap()),
            texture: self.texture.to_owned().unwrap(),
        };
    }
}

impl RenderenkoBuilded {
    pub fn render(&self) -> RenderenkoRendered {
        return RenderenkoRendered {
            render_result: render::render(
                &self.mesh,
                self.width,
                self.height,
                &self.matrix_projection,
                &self.matrix_view,
                &self.matrix_world, 
                &self.texture
            )
        }
    }
}
// #[derive(Debug, Clone)]
pub struct RenderenkoRendered {
    render_result: DynamicImage
}
impl RenderenkoRendered {
    pub fn resize(&self, nwidth: u32, nheight: u32) -> RenderenkoResized {
        let mut dst_image = Image::new(
            nwidth,nheight,
            self.render_result.pixel_type().unwrap(),
        );
        let mut resizer = Resizer::new();
        resizer.resize(&self.render_result, &mut dst_image, &ResizeOptions::new().resize_alg(ResizeAlg::Nearest)).unwrap();
        return RenderenkoResized {
            resize_result: dst_image
        }
    }
    pub fn save(&self, path: &Path) -> ImageResult<()> {
        let id:Uuid = Uuid::new_v4();
        let image_path = path.join(&format!("{}-w_{}_h_{}.png",id,self.render_result.width(),self.render_result.height()));
        return self.render_result.to_rgba8().save(&image_path);
    }
    pub fn result(&self) -> &DynamicImage {
        return &self.render_result;
    }
}
pub struct RenderenkoResized<'a> {
    resize_result: Image<'a>
}
impl RenderenkoResized<'_> {
    pub fn result(&self) -> &Image {
        return &self.resize_result;
    }
    pub fn save(&self, path: &Path) -> ImageResult<()> {
        let id:Uuid = Uuid::new_v4();
        let image_path = path.join(&format!("{}-w_{}_h_{}.png",id,&self.resize_result.width(),&self.resize_result.height()));
        return save_buffer(&image_path, self.resize_result.buffer(), self.resize_result.width(),self.resize_result.height(), ExtendedColorType::Rgba8);
        // return save(path, &self.resize_result);
    }
}