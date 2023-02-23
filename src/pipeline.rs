use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use image::{DynamicImage, GenericImageView};

const ONE_OVER_255: f32 = 1.0 / 255.;

#[inline]
fn texture_read(image_view: &DynamicImage, uv: &Vec2) -> Vec4 {
    let image_data = image_view
        .get_pixel(
            (image_view.width() as f32 * uv.x) as u32,
            (image_view.height() as f32 * uv.y) as u32,
        )
        .0;

    Vec4::new(
        image_data[0] as f32 * ONE_OVER_255,
        image_data[1] as f32 * ONE_OVER_255,
        image_data[2] as f32 * ONE_OVER_255,
        image_data[3] as f32 * ONE_OVER_255,
    )
}

pub fn run_vertex(positions: &mut [Vec3; 3], mvp: &Mat4) {
    for i in 0..3 {
        let position = mvp.mul_vec4(positions[i].extend(1.0));
        let position = position.xyz() / position.w;
        positions[i] = position;
    }
}

pub fn run_fragment(diffuse_view: &DynamicImage, uv: &Vec2, n: &Vec3) -> Vec4 {
    let diffuse = texture_read(&diffuse_view, uv);
    let light_dir = Vec3::new(0.0, 0.0, 1.0);
    let intensity = n.dot(light_dir);

    diffuse * intensity
}
