use nalgebra::Vector4;

pub fn vector_add(v1:Vector4<f32>, v2:Vector4<f32>) -> Vector4<f32> {
    return Vector4::new(v1.x+v2.x,v1.y+v2.y,v1.z+v2.z,1.0)
}
pub fn vector_subdivide(v1:Vector4<f32>, v2:Vector4<f32>) -> Vector4<f32> {
    return Vector4::new(v1.x-v2.x,v1.y-v2.y,v1.z-v2.z,1.0)
}

pub fn vector_multiply(v1:Vector4<f32>, k:f32) -> Vector4<f32> {
    return Vector4::new(v1.x* k,v1.y*k,v1.z*k,1.0);
}
pub fn vector_divide(v1:Vector4<f32>, k:f32) -> Vector4<f32> {
    return Vector4::new(v1.x/ k,v1.y/k,v1.z/k,1.0);
}
pub fn vector_dot_product(v1:Vector4<f32>, v2:Vector4<f32>) -> f32 {
    return v1.x*v2.x + v1.y*v2.y + v1.z * v2.z;
}
pub fn vector_length(v: Vector4<f32>) -> f32 {
    f32::sqrt(vector_dot_product(v, v))
}
pub fn vector_normalise(v: Vector4<f32>) -> Vector4<f32> {
    let vector_length = vector_length(v);
    return Vector4::new(v.x / vector_length, v.y / vector_length, v.z / vector_length,1.0);
}
pub fn vector_cross_product(v1:Vector4<f32>,v2:Vector4<f32>) -> Vector4<f32> {
    return Vector4::new(
        v1.y*v2.z-v1.z*v2.y,
        v1.z*v2.x-v1.x*v2.z,
        v1.x*v2.y-v1.y*v2.x,
        1.0
    ) 
}


pub fn vector_intersect_plane(plane_p: Vector4<f32>, plane_n: Vector4<f32>, line_start: Vector4<f32>, line_end: Vector4<f32>) -> (Vector4<f32>,f32) {
    let plane_n_normalised:Vector4<f32> = vector_normalise(plane_n);
    let plane_d:f32 = -vector_dot_product(plane_n_normalised, plane_p);
    let ad:f32 = vector_dot_product(line_start, plane_n_normalised);
    let bd:f32 = vector_dot_product(line_end, plane_n_normalised);
    let t:f32 = (-plane_d - ad) / (bd - ad);
    let line_start_to_end:Vector4<f32> = vector_subdivide(line_end, line_start);
    let line_to_intersect = vector_multiply(line_start_to_end, t);
    
    return (vector_add(line_start,line_to_intersect),t);   
}