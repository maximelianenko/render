// use fast_image_resize::{images::Image, IntoImageView, ResizeAlg, ResizeOptions, Resizer};

use nalgebra::{Matrix4, Vector4};

use super::{misc::{matrix::matrix_multiply_vector, triangle::{textured_triangle, triangle_clip_against_plane}, vector::{vector_add, vector_cross_product, vector_divide, vector_dot_product, vector_normalise}}, types::{MeshV4Plus, TriangleV4Plus}};

use image::{ ColorType, DynamicImage};



pub fn render(
    tris:&MeshV4Plus,
    width:f32,
    height:f32,
    matrix_projection:&Matrix4<f32>,
    matrix_view: &(Vector4<f32>,Matrix4<f32>),
    matrix_world: &Matrix4<f32>,
    texture: &DynamicImage,
    // resize: Option<ResizeImage>
) -> DynamicImage {

    let (camera,matrix_view) = matrix_view;
    let mut depth_buffer:Vec<f32> = vec![0.0; (width * height) as usize];

    let mut vec_triangles_to_raster: MeshV4Plus = Vec::new();
    
    let light_direction:Vector4<f32> = Vector4::new(0.0,0.0,-1.0,1.0);

    for tri in tris {
        let tri_translated:TriangleV4Plus = TriangleV4Plus::new(
            matrix_multiply_vector(&matrix_world, &tri.first),
            matrix_multiply_vector(&matrix_world, &tri.second),
            matrix_multiply_vector(&matrix_world, &tri.third),
            tri.dp,
            tri.uv
            
        );
        
        // let line1 = vector_subdivide(tri_translated.second, tri_translated.first);
        // let line2 = vector_subdivide(tri_translated.third,tri_translated.first);
        
        let line1 = tri_translated.second - tri_translated.first;
        let line2 = tri_translated.third - tri_translated.first;

        let normal_base = vector_cross_product(line1, line2);
        let normal = vector_normalise(normal_base);

        let camera_ray = tri_translated.first - camera;
        if  vector_dot_product(normal, camera_ray) < 0.0 {
            
            // let light_direction_length = f32::sqrt(light_direction.x*light_direction.x + light_direction.y * light_direction.y + light_direction.z*light_direction.z);
            //dot_product_of_light_direction_and_normal
            let to_light = vector_normalise(light_direction - camera_ray);
            let dp = f32::max(0.6,vector_dot_product(to_light, normal));
            // println!("{}",dp);


            let tri_viewed: TriangleV4Plus = TriangleV4Plus::new(
                matrix_multiply_vector(&matrix_view, &tri_translated.first),
                matrix_multiply_vector(&matrix_view, &tri_translated.second),
                matrix_multiply_vector(&matrix_view, &tri_translated.third),
                tri_translated.dp,
                tri_translated.uv
            );
            
            let (
                clipped_triangles,
                clipped1,
                clipped2
            ) = triangle_clip_against_plane(
                Vector4::new(0.0,0.0,0.1,1.0),
                Vector4::new(0.0,0.0,1.0,1.0),
                &tri_viewed
            );
            for n in 0..clipped_triangles {
                let clipped_tri = match n {
                    0 => clipped1.unwrap(),
                    1 => clipped2.unwrap(),
                    _ => continue
                };
    
                let mut tri_projected: TriangleV4Plus = TriangleV4Plus::new(
                    matrix_multiply_vector(matrix_projection, &clipped_tri.first),
                    matrix_multiply_vector(matrix_projection, &clipped_tri.second),
                    matrix_multiply_vector(matrix_projection, &clipped_tri.third),
                    dp,
                    clipped_tri.uv
                );
                tri_projected.uv.first.u = tri_projected.uv.first.u / tri_projected.first.w;
                tri_projected.uv.second.u = tri_projected.uv.second.u / tri_projected.second.w;
                tri_projected.uv.third.u = tri_projected.uv.third.u / tri_projected.third.w;

                tri_projected.uv.first.v = tri_projected.uv.first.v / tri_projected.first.w;
                tri_projected.uv.second.v = tri_projected.uv.second.v / tri_projected.second.w;
                tri_projected.uv.third.v = tri_projected.uv.third.v / tri_projected.third.w;

                tri_projected.uv.first.w = 1.0 / tri_projected.first.w;
                tri_projected.uv.second.w = 1.0 / tri_projected.second.w;
                tri_projected.uv.third.w = 1.0 / tri_projected.third.w;

                tri_projected = TriangleV4Plus::new(
                    vector_divide(tri_projected.first, tri_projected.first.w),
                    vector_divide(tri_projected.second, tri_projected.second.w),
                    vector_divide(tri_projected.third, tri_projected.third.w),
                    tri_projected.dp,
                    tri_projected.uv
                );

                let offset_view:Vector4<f32> = Vector4::new(1.0,1.0,0.0,1.0);
    
                tri_projected = TriangleV4Plus::new(
                    vector_add(tri_projected.first, offset_view),
                    vector_add(tri_projected.second, offset_view),
                    vector_add(tri_projected.third, offset_view),
                    tri_projected.dp,
                    tri_projected.uv
                );

                tri_projected.first.x *= 0.5 *width;
                tri_projected.first.y *= 0.5 *height;
                tri_projected.second.x *= 0.5 *width;
                tri_projected.second.y *= 0.5 *height;
                tri_projected.third.x *= 0.5 *width;
                tri_projected.third.y *= 0.5 *height;
    
                vec_triangles_to_raster.push(tri_projected);
    
            }
        }

    }
    let mut image = DynamicImage::new(
        width as u32,
        height as u32,
        ColorType::Rgba8,
    );
    for tri_to_raster in vec_triangles_to_raster {
        let mut triangles: MeshV4Plus = Vec::new();

        triangles.push(tri_to_raster);
        let mut n_new_triangles = 1;
    
        for p in 0..4 {
            // let mut n_tris_to_add = 0;
            
            while n_new_triangles > 0 {
                let test:TriangleV4Plus = triangles.first().unwrap().clone();
                triangles.remove(0);
                n_new_triangles -= 1;
    

                let (n_tris_to_add,clipped1,clipped2) = match p {
                    0 => triangle_clip_against_plane(Vector4::new(0.0, 0.0, 0.0,1.0), Vector4::new(0.0, 1.0, 0.0,1.0), &test),
                    1 => triangle_clip_against_plane(Vector4::new(0.0, height - 1.0, 0.0,1.0), Vector4::new(0.0, -1.0, 0.0,1.0), &test),
                    2 => triangle_clip_against_plane(Vector4::new(0.0, 0.0,0.0,1.0),Vector4::new(1.0, 0.0, 0.0,1.0),&test),
                    3 => triangle_clip_against_plane(Vector4::new(width - 1.0, 0.0, 0.0,1.0), Vector4::new(-1.0, 0.0, 0.0,1.0), &test),
                    _ => (0,None,None)
                };

                for w in 0..n_tris_to_add {
                    let clipped_tri:TriangleV4Plus = match w {
                        0 => clipped1.unwrap(),
                        1 => clipped2.unwrap(),
                        _ => TriangleV4Plus::default()
                    };
                
                    triangles.push(clipped_tri);
                }
            }
            n_new_triangles = triangles.len();
        };
        for tri in triangles {
            textured_triangle(
                tri.first.x as i32,tri.first.y as i32,tri.uv.first.u,tri.uv.first.v,tri.uv.first.w,
                tri.second.x as i32,tri.second.y as i32,tri.uv.second.u,tri.uv.second.v,tri.uv.second.w,
                tri.third.x as i32,tri.third.y as i32,tri.uv.third.u,tri.uv.third.v,tri.uv.third.w,
                &mut image, &texture,
                width,&mut depth_buffer,
                tri.dp
            );
        }
    }
    return image;
}