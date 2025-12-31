use glam::{Mat4, Vec3};

#[derive(Debug, Clone, Copy)]
pub enum CameraDirection {
    Focal(Vec3),
    Facing(Vec3),
}

#[derive(Debug, Clone, Copy)]
pub struct GraphicCamera {
    pub position: Vec3,
    pub direction: CameraDirection,
    pub view_field: f32,
    pub aspect_ratio: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl GraphicCamera {
    pub fn default() -> Self {
        Self {
            position: Vec3::new(-1.0,2.0, 5.0),
            direction: CameraDirection::Focal(Vec3::ZERO),
            view_field: 45.0,
            aspect_ratio: 1.0,
            z_near: 0.1,
            z_far: 100.0,
        }
    }
    pub fn view_matrix(&self) -> Mat4 {
        match self.direction {
            CameraDirection::Focal(focal_point) => {
                Mat4::look_at_rh(self.position, focal_point, Vec3::Z)
            }
            CameraDirection::Facing(facing_direction) => {
                Mat4::look_to_rh(self.position, facing_direction, Vec3::Z)
            }
        }
    }
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh_gl(
            self.view_field.to_radians(),
            self.aspect_ratio,
            self.z_near,
            self.z_far,
        )
    }
}
