use glam::{Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    view: Mat4,
    projection: Mat4,
}

impl Camera {
    pub fn new(position: Vec3, aspect: f32) -> Self {
        let target = Vec3::ZERO;
        let view = glam::Mat4::look_at_lh(position, target, glam::Vec3::Y);
        let projection =
            glam::Mat4::perspective_infinite_reverse_lh(std::f32::consts::FRAC_PI_4, aspect, 0.1);

        Self {
            position,
            target: Vec3::ZERO,
            view,
            projection,
        }
    }

    pub fn projection(&self) -> Mat4 {
        self.projection
    }

    pub fn view(&self) -> Mat4 {
        self.view
    }
}
