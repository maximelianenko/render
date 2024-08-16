use std::path::Path;

use renderenko::builder;
// s

fn main() {
    let _ = builder::Renderenko::new()
        .mesh(Path::new("data/model/default/alex.obj")).unwrap()
        .texture(Path::new("data/textures/default/maximelianenko.png")).unwrap()
        .size(500, 1000)
        .fov(35.0)
        .world(0.0,-15.0,0.0, 0.25, 1.25)
        .build()
        .render()
        .save(Path::new("out"));
        // .resize(500, 500)
        // .save(Path::new("out"));
    // let binding = binding
    //     .resize(500,500);
    // let buffer = binding.result();
    
    // println!("{:?}",buffer.buffer())
    // println!("{:?}",&buffer)
        // .save("out");
    // let mesh = load_from_obj("data/model/ferris.obj");
    // // // println!("{:?}",mesh);
    // // // let smth:Vector2<f32> = Vector2::new()
    // if mesh.is_err() {
    //     println!("{}","Error");
    //     return;
    // }
    // let tris:MeshV4Plus = mesh.unwrap();
    // let width:f32 = 80.;
    // let height:f32 = 80.;
    // let aspect_ratio:f32 = height / width;
    // let far:f32 = 1000.;
    // let near:f32 = 0.1;
    // let fov_angle:f32 = 30.;
    
    // let matrix_projection:Matrix4<f32> = matrix_projection(fov_angle, aspect_ratio, near, far);
    // // let matrix_projection:Matrix4<f32> = Matrix4::new_orthographic(0.0, 1000.0, 0.0 , 1000.0, 1.0, 1000.0);
    // // let pixmap_texture_unwrapped = Pixmap::load_png("data/textures/maximelianenko.png");
    // // let pixmap_texture = pixmap_texture_unwrapped.unwrap();
    // let texture = Reader::open("data/textures/default/ferris.png")
    //     .unwrap()
    //     .decode()
    //     .unwrap();
    // let matrix_view = matrix_view(
    //     Vector4::new(0.0,0.0,0.0,1.0),
    //     Vector4::new(0.0,3.0,0.0,1.0),
    //     Vector4::new(0.0,0.0,1.0,1.0),
    //     0.0,
    //     0.0
    // );
    // let matrix_world = matrix_world(
    //     0.0,
    //     0.0,
    //     0.0,
    //     0.36,
    //     0.75
    // );
    // // let matrix_world = matrix_world(
    // //     0.0,
    // //     -15.0,
    // //     0.0,
    // //     0.0,
    // //     2.50
    // // );
    // let matrix_world = matrix_world(
    //     0.0,
    //     -15.0,
    //     0.0,
    //     0.25,
    //     1.25
    // );
    // let resize = ResizeImage {
    //     width: 500,
    //     height: 500
    // };
    // let image = render(&tris,width,height,&matrix_projection,&matrix_view,&matrix_world,&texture);
    // let resized_image = img::resize(&image, resize);
    // img::save("out", &resized_image);
    
}
