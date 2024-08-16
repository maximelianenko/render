
use nalgebra::{Matrix4, Vector3, Vector4};

use super::{other::degree_to_rad, vector::{vector_add, vector_cross_product, vector_dot_product, vector_multiply, vector_normalise, vector_subdivide}};
pub fn matrix_world(xr:f32,yr:f32,xt:f32,yt:f32,zt:f32) -> Matrix4<f32> {
    // let mat_translate: Matrix4<f32> = matrix_translation(0.0, 0.25, 1.25);
    let matrix_world: Matrix4<f32> = Matrix4::new_rotation(
        Vector3::new(
            degree_to_rad(xr),
            degree_to_rad(yr),
            0.0
        )
    ) * matrix_translation(xt, yt, zt);
    return matrix_world;
}
pub fn matrix_view(camera: Vector4<f32>, up: Vector4<f32>,target: Vector4<f32>,pitch: f32, yaw:f32) -> (Vector4<f32>,Matrix4<f32>){
    let matrix_camera_rotation = Matrix4::new_rotation(Vector3::new(degree_to_rad(pitch),degree_to_rad(yaw),0.0));
    let look_direction:Vector4<f32> = matrix_multiply_vector(
        &matrix_camera_rotation, 
        &target
    );
    let matrix_camera:Matrix4<f32> = matrix_point_at(camera, vector_add(camera, look_direction), up);
    let matrix_view: Matrix4<f32> = matrix_quick_inverse(matrix_camera);

    return (camera,matrix_view);
}
pub fn matrix_multiply_vector(matrix: &Matrix4<f32>,vector: &Vector4<f32>) -> Vector4<f32>{
    let mut output:Vector4<f32> = Vector4::new(0.0,0.0,0.0,0.0);
    output.x = vector.x * matrix.m11 + vector.y * matrix.m21 + vector.z * matrix.m31 + vector.w * matrix.m41;
    output.y = vector.x * matrix.m12 + vector.y * matrix.m22 + vector.z * matrix.m32 + vector.w * matrix.m42;
    output.z = vector.x * matrix.m13 + vector.y * matrix.m23 + vector.z * matrix.m33 + vector.w * matrix.m43;
    output.w = vector.x * matrix.m14 + vector.y * matrix.m24 + vector.z * matrix.m34 + vector.w * matrix.m44;

    return output
}
pub fn matrix_translation(x:f32,y:f32,z:f32) -> Matrix4<f32> {
    let mat_translate:Matrix4<f32> = Matrix4::new(
        1.0,0.0,0.0,0.0,
        0.0,1.0,0.0,0.0,
        0.0,0.0,1.0,0.0,
        x,y,z,1.0,
    );
    return mat_translate
}
pub fn matrix_projection(fov:f32,aspect_ratio:f32,near:f32,far:f32) -> Matrix4<f32> {
    // let fov_angle_rad:f32 = fov * (PI/180.);
    let fov_angle_rad = 1.0 / f32::tan(fov * 0.5 / 180.0 * 3.14159);
    let mat: Matrix4<f32> = Matrix4::new(
        aspect_ratio * fov_angle_rad,0.0, 0.0,0.0,
        0.0,fov_angle_rad,0.0,0.0,
        0.0,0.0,far / (far - near),1.0,
        0.0,0.0,(-far * near) / (far - near),0.0
    );
    return mat
}
pub fn matrix_point_at(pos:Vector4<f32>,target:Vector4<f32>, up: Vector4<f32>) -> Matrix4<f32> {
    let mut new_forward: Vector4<f32>;
    new_forward = vector_subdivide(target, pos);
    new_forward = vector_normalise(new_forward);

    let a = vector_multiply(new_forward, vector_dot_product(up, new_forward));
    let mut new_up: Vector4<f32>;
    new_up = vector_subdivide(up, a);
    new_up = vector_normalise(new_up);

    let new_right:Vector4<f32>;
    new_right = vector_cross_product(new_up, new_forward);
    
    return Matrix4::new(
        new_right.x, new_right.y, new_right.z, 0.0,
        new_up.x,new_up.y,new_up.z,0.0,
        new_forward.x,new_forward.y, new_forward.z,0.0,
        pos.x,pos.y,pos.z,1.0
    )

}
pub fn matrix_quick_inverse(mat: Matrix4<f32>) -> Matrix4<f32>{
    return Matrix4::new(
        mat.m11,mat.m21,mat.m31,0.0,
        mat.m12, mat.m22, mat.m32, 0.0,
        mat.m13, mat.m23, mat.m33, 0.0,

        -(mat.m41 * mat.m11 + mat.m42 * mat.m21 + mat.m43),
        -(mat.m41 * mat.m12 + mat.m42 * mat.m22 + mat.m43),
        -(mat.m41 * mat.m13 + mat.m42 * mat.m23 + mat.m43),
        1.0
    )
}
