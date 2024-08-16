
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use nalgebra::Vector4;


use crate::rend::types::{TriangleV4Plus, UVVec};

use super::vector::{vector_dot_product, vector_intersect_plane, vector_normalise};

pub fn triangle_clip_against_plane(plane_p: Vector4<f32>,plane_n: Vector4<f32>,in_tri: &TriangleV4Plus) -> (i32, Option<TriangleV4Plus>, Option<TriangleV4Plus>) {
    let mut out_tri1: TriangleV4Plus = TriangleV4Plus::default();
    let mut out_tri2: TriangleV4Plus = TriangleV4Plus::default();
    let plane_n_normalised:Vector4<f32> = vector_normalise(plane_n);
    // let dist = |p: Vector4<f32>| -> f32 {
    //     let n = vector_normalise(p);
    //     plane_n_normalised.x * p.x + plane_n_normalised.y * p.y + plane_n_normalised.z * p.z - vector_dot_product(plane_n_normalised, plane_p)
    // };
    let dist = |p: &Vector4<f32>| -> f32 {
        // let n = vector_normalise(*p);
        plane_n.x * p.x + plane_n.y * p.y + plane_n.z * p.z - vector_dot_product(plane_n, plane_p)
    };

    let mut inside_points: Vec<Vector4<f32>> = Vec::new();
    let mut outside_points: Vec<Vector4<f32>> = Vec::new();

    let mut inside_texture: Vec<UVVec> = Vec::new();
    let mut outside_texture: Vec<UVVec> = Vec::new();

    let d0 = dist(&in_tri.first);
    let d1 = dist(&in_tri.second);
    let d2 = dist(&in_tri.third);

    if d0 >= 0.0 {
        inside_points.push(in_tri.first);
        inside_texture.push(in_tri.uv.first)
    } else {
        outside_points.push(in_tri.first);
        outside_texture.push(in_tri.uv.first)
    }
    if d1 >= 0.0 {
        inside_points.push(in_tri.second);
        inside_texture.push(in_tri.uv.second)
    } else {
        outside_points.push(in_tri.second);
        outside_texture.push(in_tri.uv.second)
    }
    if d2 >= 0.0 {
        inside_points.push(in_tri.third);
        inside_texture.push(in_tri.uv.third)
    } else {
        outside_points.push(in_tri.third);
        outside_texture.push(in_tri.uv.third)
    }

    if inside_points.len() == 0 {
        return (0,None,None);
    }
    if inside_points.len() == 3 {
        out_tri1 = *in_tri;
        return (1,Some(out_tri1),None);
    }
    if inside_points.len() == 1 && outside_points.len() == 2 {
        let mut t:f32;
        out_tri1.first = inside_points[0];
        out_tri1.uv.first = inside_texture[0];
        (out_tri1.second,t) = vector_intersect_plane(plane_p, plane_n_normalised, inside_points[0], outside_points[0]);
        out_tri1.uv.second.u = t * (outside_texture[0].u - inside_texture[0].u) + inside_texture[0].u;
        out_tri1.uv.second.v = t * (outside_texture[0].v - inside_texture[0].v) + inside_texture[0].v;
        out_tri1.uv.second.w = t * (outside_texture[0].w - inside_texture[0].w) + inside_texture[0].w;


        (out_tri1.third,t) = vector_intersect_plane(plane_p, plane_n_normalised, inside_points[0], outside_points[1]);
        out_tri1.uv.third.u = t * (outside_texture[1].u - inside_texture[0].u) + inside_texture[0].u;
        out_tri1.uv.third.v = t * (outside_texture[1].v - inside_texture[0].v) + inside_texture[0].v;
        out_tri1.uv.third.w = t * (outside_texture[1].w - inside_texture[0].w) + inside_texture[0].w;

        return (1,Some(out_tri1),None);
    }

    if inside_points.len() == 2 && outside_points.len() == 1 {
        out_tri1.first = inside_points[0];
        out_tri1.second = inside_points[1];

        out_tri1.uv.first = inside_texture[0];
        out_tri1.uv.second = inside_texture[1];

        let mut t:f32;
        (out_tri1.third,t) = vector_intersect_plane(plane_p, plane_n_normalised, inside_points[0], outside_points[0]);
        out_tri1.uv.third.u = t * (outside_texture[0].u - inside_texture[0].u) + inside_texture[0].u;
        out_tri1.uv.third.v = t * (outside_texture[0].v - inside_texture[0].v) + inside_texture[0].v;
        out_tri1.uv.third.w = t * (outside_texture[0].w - inside_texture[0].w) + inside_texture[0].w;

        out_tri2.first = inside_points[1];
        out_tri2.second = out_tri1.third;
        out_tri2.uv.first = inside_texture[1];
        out_tri2.uv.second = out_tri1.uv.third;

        (out_tri2.third,t) = vector_intersect_plane(plane_p, plane_n_normalised, inside_points[1], outside_points[0]);
        out_tri2.uv.third.u = t * (outside_texture[0].u - inside_texture[1].u) + inside_texture[1].u;
        out_tri2.uv.third.v = t * (outside_texture[0].v - inside_texture[1].v) + inside_texture[1].v;
        out_tri2.uv.third.w = t * (outside_texture[0].w - inside_texture[1].w) + inside_texture[1].w;
        return (2,Some(out_tri1),Some(out_tri2));
    }

    return (0,None,None);
}

use std::mem::swap;


pub fn textured_triangle(
    mut x1:i32,mut y1:i32,mut u1:f32,mut v1:f32, mut w1:f32,
    mut x2:i32,mut y2:i32,mut u2:f32,mut v2:f32, mut w2:f32,
    mut x3:i32,mut y3:i32,mut u3:f32,mut v3:f32, mut w3:f32,
    image: &mut DynamicImage,texture: &DynamicImage, width:f32,
    depth_buffer: &mut Vec<f32>,
    dp: f32
) {
    let texture_height = texture.height() as f32;
    let texture_width = texture.width() as f32;
    if y2 < y1 {
        swap(&mut y1, &mut y2);
        swap(&mut x1, &mut x2);
        swap(&mut u1, &mut u2);
        swap(&mut v1, &mut v2);
        swap(&mut w1, &mut w2);
    }

    if y3 < y1 {
        swap(&mut y1, &mut y3);
        swap(&mut x1, &mut x3);
        swap(&mut u1, &mut u3);
        swap(&mut v1, &mut v3);
        swap(&mut w1, &mut w3);
    }

    if y3 < y2 {
        swap(&mut y2, &mut y3);
        swap(&mut x2, &mut x3);
        swap(&mut u2, &mut u3);
        swap(&mut v2, &mut v3);
        swap(&mut w2, &mut w3);
    }

    let dy1 = y2 - y1;
    let dx1 = x2 - x1;
    let dv1 = v2 - v1;
    let du1 = u2 - u1;
    let dw1 = w2 - w1;

    let dy2 = y3 - y1;
    let dx2 = x3 - x1;
    let dv2 = v3 - v1;
    let du2 = u3 - u1;
    let dw2 = w3 - w1;

    let mut tex_u;
    let mut tex_v;
    let mut tex_w;

    let mut dax_step = 0.0;
    let mut dbx_step = 0.0;
    let mut du1_step = 0.0;
    let mut dv1_step = 0.0;
    let mut du2_step = 0.0;
    let mut dv2_step = 0.0;
    let mut dw1_step = 0.0;
    let mut dw2_step = 0.0;

    if dy1 != 0 {
        dax_step = dx1 as f32 / dy1.abs() as f32;
    }
    if dy2 != 0 {
        dbx_step = dx2 as f32 / dy2.abs() as f32;
    }

    if dy1 != 0 {
        du1_step = du1 / dy1.abs() as f32;
        dv1_step = dv1 / dy1.abs() as f32;
        dw1_step = dw1 / dy1.abs() as f32;
    }

    if dy2 != 0 {
        du2_step = du2 / dy2.abs() as f32;
        dv2_step = dv2 / dy2.abs() as f32;
        dw2_step = dw2 / dy2.abs() as f32;
    }
    if dy1 != 0 {
        for i in y1..=y2 {
            let mut ax = x1 + ((i - y1) as f32 * dax_step) as i32;
            let mut bx = x1 + ((i - y1) as f32 * dbx_step) as i32;

            let mut tex_su = u1 + (i - y1) as f32 * du1_step;
            let mut tex_sv = v1 + (i - y1) as f32 * dv1_step;
            let mut tex_sw = w1 + (i - y1) as f32 * dw1_step;

            let mut tex_eu = u1 + (i - y1) as f32 * du2_step;
            let mut tex_ev = v1 + (i - y1) as f32 * dv2_step;
            let mut tex_ew = w1 + (i - y1) as f32 * dw2_step;

            if ax > bx {
                swap(&mut ax, &mut bx);
                swap(&mut tex_su, &mut tex_eu);
                swap(&mut tex_sv, &mut tex_ev);
                swap(&mut tex_sw, &mut tex_ew);
            }

            let tstep = 1.0 / (bx - ax) as f32;
            let mut t = 0.0;

            for j in ax..bx {
                let i = i as f32;
                let j = j as f32;
                tex_u = (1.0 - t) * tex_su + t * tex_eu;
                tex_v = (1.0 - t) * tex_sv + t * tex_ev;
                tex_w = (1.0 - t) * tex_sw + t * tex_ew;
                // println!("{}", tex_w);
                // println!("{}",texture.width());
                if tex_w > depth_buffer[(i * width + j) as usize] {
                    // let pixel = pixmap.get_mut((i * width + j) as usize).unwrap();
                    // texture.get_pixel(0, 0);
                    let texture_pixel = texture.get_pixel(((tex_u / tex_w) * texture_width) as u32, ((1.0 - (tex_v / tex_w)) * texture_height) as u32);
                    if texture_pixel[3] != 0 {
                        let texture_pixel_red = (texture_pixel[0] as f32 * dp) as u8;
                        let texture_pixel_blue = (texture_pixel[2] as f32 * dp) as u8;
                        let texture_pixel_green = (texture_pixel[1] as f32 * dp) as u8;
                        let texture_pixel_alpha = texture_pixel[3];
                        // let texture_pixel_dp = PremultipliedColorU8::from_rgba(texture_pixel_red,texture_pixel_green,texture_pixel_blue,texture_pixel_alpha).unwrap();
                        image.put_pixel(j as u32 , i as u32, Rgba([texture_pixel_red,texture_pixel_green,texture_pixel_blue,texture_pixel_alpha]));
                        // *pixel = texture_pixel_dp;
                        depth_buffer[(i * width + j) as usize] = tex_w;
                    }

                }
                t += tstep;
            }
        }
    }

    let dy1 = y3 - y2;
    let dx1 = x3 - x2;
    let dv1 = v3 - v2;
    let du1 = u3 - u2;
    let dw1 = w3 - w2;

    if dy1 != 0 {
        dax_step = dx1 as f32 / dy1.abs() as f32;
    }
    if dy2 != 0 {
        dbx_step = dx2 as f32 / dy2.abs() as f32;
    }

    du1_step = 0.0;
    dv1_step = 0.0;
    if dy1 != 0 {
        du1_step = du1 / dy1.abs() as f32;
        dv1_step = dv1 / dy1.abs() as f32;
        dw1_step = dw1 / dy1.abs() as f32;
    }

    if dy1 != 0 {
        for i in y2..=y3 {
            let mut ax = x2 + ((i - y2) as f32 * dax_step) as i32;
            let mut bx = x1 + ((i - y1) as f32 * dbx_step) as i32;

            let mut tex_su = u2 + (i - y2) as f32 * du1_step;
            let mut tex_sv = v2 + (i - y2) as f32 * dv1_step;
            let mut tex_sw = w2 + (i - y2) as f32 * dw1_step;

            let mut tex_eu = u1 + (i - y1) as f32 * du2_step;
            let mut tex_ev = v1 + (i - y1) as f32 * dv2_step;
            let mut tex_ew = w1 + (i - y1) as f32 * dw2_step;

            if ax > bx {
                swap(&mut ax, &mut bx);
                swap(&mut tex_su, &mut tex_eu);
                swap(&mut tex_sv, &mut tex_ev);
                swap(&mut tex_sw, &mut tex_ew);
            }

            let tstep = 1.0 / (bx - ax) as f32;
            let mut t = 0.0;

            for j in ax..bx {
                let i = i as f32;
                let j = j as f32;
                tex_u = (1.0 - t) * tex_su + t * tex_eu;
                tex_v = (1.0 - t) * tex_sv + t * tex_ev;
                tex_w = (1.0 - t) * tex_sw + t * tex_ew;

                if tex_w > depth_buffer[(i * width + j) as usize] {
                    // let pixel = pixmap.get_mut((i * width + j) as usize).unwrap();
                    let texture_pixel = texture.get_pixel(((tex_u / tex_w) * texture_width) as u32, ((1.0 - (tex_v / tex_w)) * texture_height) as u32);
                    if texture_pixel[3] != 0 {
                        let texture_pixel_red = (texture_pixel[0] as f32 * dp) as u8;
                        let texture_pixel_blue = (texture_pixel[2] as f32 * dp) as u8;
                        let texture_pixel_green = (texture_pixel[1] as f32 * dp) as u8;
                        let texture_pixel_alpha = texture_pixel[3];
                        // let texture_pixel_dp = PremultipliedColorU8::from_rgba(texture_pixel_red,texture_pixel_green,texture_pixel_blue,texture_pixel_alpha).unwrap();
                        // *pixel = texture_pixel_dp;
                        image.put_pixel(j as u32 , i as u32, Rgba([texture_pixel_red,texture_pixel_green,texture_pixel_blue,texture_pixel_alpha]));
                        depth_buffer[(i * width + j) as usize] = tex_w;
                    }
                }
                t += tstep;
            }
        }
    }
    // let mut result_buf = BufWriter::new(Vec::new());
    // PngEncoder::new(&mut result_buf)
    // .write_image(
    //     image.as_rgba8().unwrap(),
    //     width as u32,
    //     height as u32,
    //     ExtendedColorType::Rgba8,
    // )
    // .unwrap();

}