pub mod camera;
pub mod canvas;
pub mod model;
pub mod pipeline;

use crate::canvas::Canvas;
use camera::Camera;
use glam::{IVec2, Vec2, Vec3};
use image::DynamicImage;
use model::Model;
use pipeline::{run_fragment, run_vertex}; //, DefaultVertexShader, MutVertices, Shader};

const RENDER_WIDTH: usize = 1000;
const RENDER_HEIGHT: usize = 1000;

const HALF_RENDER_WIDTH: usize = RENDER_WIDTH / 2;
const HALF_RENDER_HEIGHT: usize = RENDER_HEIGHT / 2;

const ASPECT_RATIO: f32 = RENDER_WIDTH as f32 / RENDER_HEIGHT as f32;

// Return the barycentric coord of p in a,b,c
pub fn barycentric(p: &IVec2, a: &IVec2, b: &IVec2, c: &IVec2) -> Vec3 {
    let u = Vec3::new((c.x - a.x) as f32, (b.x - a.x) as f32, (a.x - p.x) as f32).cross(Vec3::new(
        (c.y - a.y) as f32,
        (b.y - a.y) as f32,
        (a.y - p.y) as f32,
    ));

    if u.z.abs() < 1.0 {
        return Vec3::new(-1.0, 0.0, -1.0);
    }

    let divisor = 1.0 / u.z;

    Vec3::new(1.0 - (u.x + u.y) * divisor, u.y * divisor, u.x * divisor)
}

pub fn draw_triangle(
    canvas: &mut Canvas,
    positions: &[Vec3; 3],
    uvs: &[Vec2; 3],
    normals: &[Vec3; 3],
    diffuse_view: &DynamicImage,
) {
    // Convert vertex positions to screenspace
    // for rasterizing
    let mut ss_positions = [IVec2::ZERO; 3];
    for i in 0..3 {
        ss_positions[i].x =
            (HALF_RENDER_WIDTH as f32 * positions[i].x + HALF_RENDER_WIDTH as f32) as i32;
        ss_positions[i].y =
            (HALF_RENDER_HEIGHT as f32 * positions[i].y + HALF_RENDER_HEIGHT as f32) as i32;
    }

    let [a, b, c] = ss_positions;

    let mut bbox_min = IVec2::new((canvas.width - 1) as i32, (canvas.height - 1) as i32);
    let mut bbox_max = IVec2::new(0, 0);
    let clamp = IVec2::new((canvas.width - 1) as i32, (canvas.height - 1) as i32);

    // Find the min and max bounds of the given triangle
    bbox_min.x = bbox_min.x.min(a.x.min(b.x.min(c.x))).max(0);
    bbox_min.y = bbox_min.y.min(a.y.min(b.y.min(c.y))).max(0);

    bbox_max.x = bbox_max.x.max(a.x.max(b.x.max(c.x))).min(clamp.x);
    bbox_max.y = bbox_max.y.max(a.y.max(b.y.max(c.y))).min(clamp.y);

    // For every pixel in the bounding box, if it is
    // contained in the triangle, draw it.
    for x in bbox_min.x..bbox_max.x {
        for y in bbox_min.y..bbox_max.y {
            let bc = barycentric(&IVec2::new(x, y), &a, &b, &c);

            if bc.min_element() < 0.0 {
                continue;
            }

            // Interpolate depth
            let z = (positions[0].z * bc.x) + (positions[1].z * bc.y) + (positions[2].z * bc.z);

            // Depth test
            if z >= canvas.read_depth(x as usize, y as usize) {
                // Interpolate uv
                let mut uv = (uvs[0] * bc.x) + (uvs[1] * bc.y) + (uvs[2] * bc.z);
                uv.y = 1.0 - uv.y;

                // Interpolate normal
                let n = (normals[0] * bc.x) + (normals[1] * bc.y) + (normals[2] * bc.z);

                let color = run_fragment(&diffuse_view, &uv, &n);

                // Convert to 0-255 range
                let color = [
                    (color.x * 255.) as u8,
                    (color.y * 255.) as u8,
                    (color.z * 255.) as u8,
                ];

                canvas
                    .write_pixel(x as usize, y as usize, z, &color)
                    .expect("Couldn't write pixel color in draw_triangle");
            }
        }
    }
}

fn main() {
    let mut canvas = Canvas::new(RENDER_WIDTH, RENDER_HEIGHT);
    let camera = Camera::new(Vec3::new(100.0, 0.0, 300.0), ASPECT_RATIO);
    let model = Model::from_file("head.obj");

    let diffuse_view = image::open("head_diffuse.tga").expect("Couldn't open diffuse texture");

    let mvp = camera.projection() * camera.view() * model.transformation;

    model.faces.iter().for_each(|face| {
        let mut positions = model.face_vertices(&face);
        let uvs = model.face_uvs(&face);
        let normals = model.face_normals(&face);

        let normal = ((positions[2] - positions[0]).cross(positions[1] - positions[0])).normalize();

        // Backface culling
        if normal.dot(camera.position - positions[0]) >= 0.0 {
            return;
        }

        run_vertex(&mut positions, &mvp);

        draw_triangle(&mut canvas, &positions, &uvs, &normals, &diffuse_view);
    });

    canvas.write_to_file("result.ppm");
}
