use std::f32::consts::PI;

use glam::{Mat4, Vec2, Vec3, Vec4};
use glow::HasContext;

use crate::graphic::{
    camera::GraphicCamera,
    drawable::drawable::GraphicDrawable,
    graphic::GraphicMVPMatrix,
    program::{PROGRAM_MANAGER, ProgramId},
};

#[derive(Debug, Clone)]
pub struct DrawableArrow {
    program: glow::NativeProgram,
    color: [f32; 4],
    line_width: f32,
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    ebo: glow::Buffer,
}

impl DrawableArrow {
    pub fn new(gl: &glow::Context) -> Self {
        unsafe {
            let vao = gl
                .create_vertex_array()
                .expect("Unable to create vertex array.");
            let vbo = gl.create_buffer().expect("Unable to create buffer");
            let ebo = gl.create_buffer().expect("Unable to create buffer");

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            // Original Position (f32;3)
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 7 * size_of::<f32>() as i32, 0);
            gl.enable_vertex_attrib_array(0);

            gl.bind_vertex_array(None);

            Self {
                program: PROGRAM_MANAGER
                    .get_program(gl, ProgramId::DrawableArrow)
                    .expect("Drawable Arrow program not created"),
                color: [1.0f32; 4],
                line_width: 1.0,
                vao,
                vbo,
                ebo,
            }
        }
    }

    pub fn set_points(&mut self, gl: &glow::Context, start: Vec3, end: Vec3) {
        // let dir = (start - end).normalize();

        let vertices = [
            start.x, start.y, start.z, // Start
            end.x, end.y, end.z, // End
        ];

        let indices = [0, 1];
        // let indices = [0, 1, 1, 2, 1, 3];

        unsafe {
            gl.bind_vertex_array(Some(self.vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            let u8_buffer = bytemuck::cast_slice(&vertices[..]);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            let u8_buffer = bytemuck::cast_slice(&indices[..]);
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);

            gl.bind_vertex_array(None);
        }
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    pub fn set_line_width(&mut self, line_width: f32) {
        self.line_width = line_width;
    }

    pub fn set_program(&mut self, gl: &glow::Context, id: &ProgramId) -> Result<(), String> {
        let program = PROGRAM_MANAGER.get_program(gl, id.clone());
        if program.is_none() {
            return Err(format!("Program {} is None.", id));
        }
        self.program = program.unwrap();
        Ok(())
    }
}

impl GraphicDrawable for DrawableArrow {
    fn draw(&self, gl: &glow::Context, camera: &GraphicCamera) {
        unsafe {
            gl.use_program(Some(self.program));
            let mvp_transform = GraphicMVPMatrix::from_camera(camera, Mat4::IDENTITY);
            mvp_transform.assign_gl_program(gl, self.program);

            let color_location = gl.get_uniform_location(self.program, "color");
            if let Some(loc) = color_location {
                gl.uniform_4_f32_slice(Some(&loc), &self.color);
            }

            gl.line_width(self.line_width);

            gl.bind_vertex_array(Some(self.vao));
            gl.depth_func(glow::LEQUAL);
            gl.draw_elements(glow::LINES, 2, glow::UNSIGNED_INT, 0);

            gl.bind_vertex_array(None);
        }
    }

    fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo);
            gl.delete_buffer(self.ebo);
        }
    }
}
