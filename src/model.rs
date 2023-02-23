use glam::{Mat4, Vec2, Vec3};

use std::fs::File;
use std::io::{BufRead, BufReader};

// Holds indices into the main
// vertex, uv, and normal buffers
pub struct Face {
    pub vertices: [usize; 3],
    pub uvs: [usize; 3],
    pub normals: [usize; 3],
}

pub struct Model {
    pub vertices: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
    pub normals: Vec<Vec3>,
    pub faces: Vec<Face>,

    pub transformation: Mat4,
}

impl Model {
    pub fn from_file(path: &str) -> Self {
        let file = File::open(path).expect("Couldn't open file");
        let reader = BufReader::new(file);

        let mut vertices = Vec::<Vec3>::new();
        let mut uvs = Vec::<Vec2>::new();
        let mut normals = Vec::<Vec3>::new();
        let mut faces = Vec::<Face>::new();

        reader.lines().for_each(|line| {
            let line = line.unwrap();
            let mut line_it = line.split(' ');

            match line_it.next() {
                Some("v") => {
                    let mut vertex = [0.0; 3];
                    line_it.enumerate().for_each(|(i, v)| {
                        vertex[i] = v.parse::<f32>().unwrap_or_default();
                    });
                    vertices.push(Vec3::from_array(vertex));
                }
                Some("vt") => {
                    _ = line_it.next(); // Double spaces in .obj file
                    let mut uv = [0.0; 2];
                    line_it.enumerate().take(2).for_each(|(i, v)| {
                        uv[i] = v.parse::<f32>().unwrap_or_default();
                    });
                    uvs.push(Vec2::from_array(uv));
                }
                Some("vn") => {
                    line_it.next(); // Double spaces in .obj file
                    let mut normal = [0.0; 3];
                    line_it.enumerate().for_each(|(i, v)| {
                        normal[i] = v.parse::<f32>().unwrap_or_default();
                    });
                    normals.push(Vec3::from_array(normal));
                }
                Some("f") => {
                    let mut face = Face {
                        vertices: [0; 3],
                        uvs: [0; 3],
                        normals: [0; 3],
                    };

                    line_it.enumerate().for_each(|(i, v)| {
                        let mut v_i = v.split('/');
                        face.vertices[i] = v_i.next().unwrap().parse::<usize>().unwrap() - 1;
                        face.uvs[i] = v_i.next().unwrap().parse::<usize>().unwrap() - 1;
                        face.normals[i] = v_i.next().unwrap().parse::<usize>().unwrap() - 1;
                    });

                    faces.push(face);
                }
                _ => (),
            }
        });

        Self {
            vertices,
            uvs,
            normals,
            faces,
            transformation: Mat4::from_scale_rotation_translation(
                Vec3::new(100., 100., 100.),
                glam::Quat::IDENTITY,
                Vec3::ZERO,
            ),
        }
    }

    pub fn face_vertices(&self, face: &Face) -> [Vec3; 3] {
        [
            self.vertices[face.vertices[0]],
            self.vertices[face.vertices[1]],
            self.vertices[face.vertices[2]],
        ]
    }

    pub fn face_uvs(&self, face: &Face) -> [Vec2; 3] {
        [
            self.uvs[face.uvs[0]],
            self.uvs[face.uvs[1]],
            self.uvs[face.uvs[2]],
        ]
    }

    pub fn face_normals(&self, face: &Face) -> [Vec3; 3] {
        [
            self.normals[face.normals[0]],
            self.normals[face.normals[1]],
            self.normals[face.normals[2]],
        ]
    }
}
