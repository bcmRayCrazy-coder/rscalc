use glam::{Mat4, Vec2, Vec3};
use glow::HasContext;

use crate::graphic::{
    camera::GraphicCamera,
    drawable::{arrow::DrawableArrow, drawable::GraphicDrawable, line::DrawableLine, polygon::DrawablePolygon},
    program::{PROGRAM_MANAGER, ProgramId},
};

#[derive(Debug, Clone, Copy)]
pub struct GraphicMVPMatrix {
    model: Mat4,
    view: Mat4,
    projection: Mat4,
}

impl GraphicMVPMatrix {
    pub fn from_camera(camera: &GraphicCamera, model: Mat4) -> Self {
        Self {
            model,
            view: camera.view_matrix(),
            projection: camera.projection_matrix(),
        }
    }

    pub fn assign_gl_program(&self, gl: &glow::Context, program: glow::NativeProgram) {
        unsafe {
            let model_location = gl.get_uniform_location(program, "model");
            let view_location = gl.get_uniform_location(program, "view");
            let projection_location = gl.get_uniform_location(program, "projection");

            let model_array: [[f32; 4]; 4] = self.model.to_cols_array_2d();
            let view_array: [[f32; 4]; 4] = self.view.to_cols_array_2d();
            let projection_array: [[f32; 4]; 4] = self.projection.to_cols_array_2d();

            gl.uniform_matrix_4_f32_slice(model_location.as_ref(), false, &model_array.concat());
            gl.uniform_matrix_4_f32_slice(view_location.as_ref(), false, &view_array.concat());
            gl.uniform_matrix_4_f32_slice(
                projection_location.as_ref(),
                false,
                &projection_array.concat(),
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct GraphicUpdateOptions {
    pub drag_motion: Vec2,
    pub drag_button: Option<egui::PointerButton>,
}

pub struct GraphicRenderer {
    pub camera: GraphicCamera,

    pub drag_scale: f32,

    last_frame_time: std::time::Instant,
    frame_time: f32,
    depth_buffer: Option<glow::Renderbuffer>,

    test_line: DrawableLine,
    test_polygon: DrawablePolygon,
    test_arrow: DrawableArrow,
}

impl GraphicRenderer {
    pub fn default<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
        let gl = cc.gl.as_ref().expect("Unable to use gl");

        let mut test_line = DrawableLine::new(gl);
        test_line.set_color([1.0, 0.0, 1.0, 1.0]);
        test_line.set_points(gl, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        test_line.set_program(gl, &ProgramId::Default).unwrap();

        let mut test_polygon = DrawablePolygon::new(gl);
        test_polygon.set_color([1.0, 0.0, 0.0, 1.0]);
        test_polygon.set_verts(
            gl,
            &vec![
                Vec3::new(0.0, 0.5, 1.0),
                Vec3::new(0.3, 0.3, 1.0),
                Vec3::new(0.5, 0.0, 1.0),
                Vec3::new(0.3, -0.3, 1.0),
                Vec3::new(0.0, -0.5, 1.0),
                Vec3::new(-0.3, -0.3, 1.0),
                Vec3::new(-0.5, 0.0, 1.0),
                Vec3::new(-0.3, 0.3, 1.0),
            ],
        );

        let mut test_arrow = DrawableArrow::new(gl);
        test_arrow.set_color([0.0,0.0,1.0,1.0]);
        test_arrow.set_line_width(4.0);
        test_arrow.set_points(gl, Vec3::new(0.0, 0.0, -1.0), Vec3::new(1.0, 1.5, 0.5));

        Some(Self {
            camera: GraphicCamera::default(),
            drag_scale: 0.05,
            last_frame_time: std::time::Instant::now(),
            frame_time: 0.0f32,
            depth_buffer: None,
            test_line,
            test_polygon,
            test_arrow
        })
    }

    pub fn paint(&mut self, gl: &glow::Context, opt: GraphicUpdateOptions) {
        let now = std::time::Instant::now();
        let elapsed_time = now.duration_since(self.last_frame_time).as_secs_f32();
        self.frame_time += elapsed_time;

        // Update
        // let green_val = ((self.frame_time).sin() / 2.0) + 0.5;
        let mut end_point = self.test_line.get_end_point();
        end_point.x = (self.frame_time * 3.0).cos() / 2.0;
        end_point.y = (self.frame_time * 3.0).sin() / 2.0;
        self.test_line
            .set_points(gl, self.test_line.get_start_point(), end_point);
        self.test_line.set_line_width((self.frame_time) % 6.0);
        // self.camera.position.z = (self.frame_time * 0.5).cos() * -10.0;

        if let Some(drag_button) = opt.drag_button {
            match drag_button {
                egui::PointerButton::Middle => {
                    self.camera.position.x += opt.drag_motion.x * self.drag_scale * -1.0;
                    self.camera.position.z += opt.drag_motion.y * self.drag_scale;
                }
                _ => {}
            }
        }
        self.camera.direction = super::camera::CameraDirection::Focal(Vec3::new(0.0, 0.0, 0.0));
        // self.camera.direction =
        //     super::camera::CameraDirection::Focal(end_point.clone().normalize());

        // Draw
        unsafe {
            self.ensure_depth_buffer(gl);

            gl.clear_color(1.0, 1.0, 1.0, 1.0);
            gl.clear_depth(1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            // 正交投影
            // Mat4::orthographic_rh_gl(left, right, bottom, top, near, far);

            gl.enable(glow::DEPTH_TEST);
            gl.depth_mask(true);
            gl.depth_range_f32(0.0, 1.0);

            self.test_polygon.draw(gl, &self.camera);
            self.test_line.draw(gl, &self.camera);
            self.test_arrow.draw(gl, &self.camera);

            gl.use_program(None);
        }
        self.last_frame_time = now;
    }

    pub fn destroy(&mut self, gl: &glow::Context) {
        PROGRAM_MANAGER.delete_all_program(gl);
        unsafe {
            // gl.delete_vertex_array(self.vao);

            if let Some(rb) = self.depth_buffer.take() {
                gl.delete_renderbuffer(rb);
            }
        }
        self.test_line.destroy(gl);
        self.test_polygon.destroy(gl);
    }

    fn ensure_depth_buffer(&mut self, gl: &glow::Context) {
        unsafe {
            let mut viewport = [0; 4];
            gl.get_parameter_i32_slice(glow::VIEWPORT, &mut viewport);
            if self.depth_buffer.is_none() || {
                let mut current_width = 0;
                let mut current_height = 0;
                if let Some(rb) = self.depth_buffer {
                    gl.bind_renderbuffer(glow::RENDERBUFFER, Some(rb));
                    current_width = gl.get_renderbuffer_parameter_i32(
                        glow::RENDERBUFFER,
                        glow::RENDERBUFFER_WIDTH,
                    );
                    current_height = gl.get_renderbuffer_parameter_i32(
                        glow::RENDERBUFFER,
                        glow::RENDERBUFFER_HEIGHT,
                    );
                    gl.bind_renderbuffer(glow::RENDERBUFFER, None);
                }
                current_width != viewport[2] || current_height != viewport[3]
            } {
                if let Some(rb) = self.depth_buffer.take() {
                    gl.delete_renderbuffer(rb);
                }

                let depth_rb = gl
                    .create_renderbuffer()
                    .expect("Couldn't create render buffer.");
                gl.bind_renderbuffer(glow::RENDERBUFFER, Some(depth_rb));
                gl.renderbuffer_storage(
                    glow::RENDERBUFFER,
                    glow::DEPTH_COMPONENT24,
                    viewport[2],
                    viewport[3],
                );

                gl.framebuffer_renderbuffer(
                    glow::FRAMEBUFFER,
                    glow::DEPTH_ATTACHMENT,
                    glow::RENDERBUFFER,
                    Some(depth_rb),
                );

                gl.bind_renderbuffer(glow::RENDERBUFFER, None);
                self.depth_buffer = Some(depth_rb);
            }
        }
    }
}
