use crate::types::{Mat4, Mat4x1, Vec3};

#[inline]
pub fn translate(mat: &mut Mat4x1, dx: f32, dy: f32, dz: f32) {
    *mat = Mat4::new(
        1.0, 0.0, 0.0, dx,
        0.0, 1.0, 0.0, dy,
        0.0, 0.0, 1.0, dz,
        0.0, 0.0, 0.0, 1.0,
    ) * *mat;
}

#[inline]
pub fn mat4x1_to_vec3(mat4x1: &Mat4x1) -> Vec3 {
    if mat4x1[3] != 1.0 {
        Vec3::new(
            mat4x1[0] / mat4x1[3],
            mat4x1[1] / mat4x1[3],
            mat4x1[2],
        )
    } else {
        Vec3::new(
            mat4x1[0],
            mat4x1[1],
            mat4x1[2],
        )
    }
}

#[inline]
pub fn vec3_to_mat4x1(vec3: &Vec3) -> Mat4x1 {
    Mat4x1::new(
        vec3[0],
        vec3[1],
        vec3[2],
        1.0,
    )
}