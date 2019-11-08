#![allow(dead_code)]
use nannou::math::cgmath::{self, Matrix4, Rad, Vector3, Vector4};
use nannou::prelude::*;

pub enum CamMode {
    Perspective,
    Orthographic,
}
pub struct Camera {
    mode: CamMode,
    z_min: f32,
    z_max: f32,
    screen_distance: f32,
    window_size: (f32, f32),
}

impl Camera {
    pub fn new(window_size: (f32, f32)) -> Self {
        let mode = CamMode::Perspective;
        let z_min = 0.01;
        let z_max = 1000.0;
        let screen_distance = 300.0;

        Self {
            mode,
            z_min,
            z_max,
            screen_distance,
            window_size,
        }
    }

    /// transform given position into camera coordinate
    pub fn projection(&self, position: Point3<f32>) -> Vector4<f32> {
        let (w, h) = self.window_size;
        let proj = cgmath::perspective(
            Rad(std::f32::consts::FRAC_PI_2),
            w / h,
            self.z_min,
            self.z_max,
        );
        let view = Matrix4::look_at(
            cgmath::Point3::new(0.3, 0.3, 1.0),
            cgmath::Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, -1.0, 0.0),
        );

        proj * view * Vector4::from((position.x, position.y, position.z, 1.0))
    }

    pub fn get_window_w(&self) -> f32 {
        self.window_size.0
    }

    pub fn get_window_h(&self) -> f32 {
        self.window_size.1
    }
}
