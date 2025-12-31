use glam::{Mat4, Vec3};
use glow::HasContext;

use crate::graphic::{
    camera::GraphicCamera,
    drawable::drawable::GraphicDrawable,
    graphic::GraphicMVPMatrix,
    program::{PROGRAM_MANAGER, ProgramId},
};

#[derive(Debug, Clone)]
pub struct DrawableLine {
    color: [f32; 4],
    line_width: f32,
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    vertices: [f32; 6],
    program: glow::NativeProgram,
}

impl DrawableLine {
    pub fn new(gl: &glow::Context) -> Self {
        unsafe {
            let vao = gl
                .create_vertex_array()
                .expect("Unable to create vertex array");
            let vbo = gl.create_buffer().expect("Unable to create buffer");

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(0);

            gl.bind_vertex_array(None);

            Self {
                color: [1.0f32; 4],
                line_width: 1.0,
                vao,
                vbo,
                vertices: [0.0f32; 6],
                program: PROGRAM_MANAGER
                    .get_program(gl, ProgramId::DrawableLine)
                    .expect("Drawable Line program not created"),
            }
        }
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    pub fn set_line_width(&mut self, line_width: f32) {
        self.line_width = line_width
    }

    pub fn set_points(&mut self, gl: &glow::Context, start: Vec3, end: Vec3) {
        let mut vertices = [0.0f32; 6];
        vertices[..3].copy_from_slice(&start.to_array());
        vertices[3..].copy_from_slice(&end.to_array());
        unsafe {
            gl.bind_vertex_array(Some(self.vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            let u8_vertices = bytemuck::cast_slice(&vertices[..]);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_vertices, glow::STATIC_DRAW);

            gl.bind_vertex_array(None);
        }
        self.vertices = vertices;
    }

    pub fn set_program(&mut self, gl: &glow::Context, id: &ProgramId) -> Result<(), String> {
        let program = PROGRAM_MANAGER.get_program(gl, id.clone());
        if program.is_none() {
            return Err(format!("Program {} is None.", id));
        }
        self.program = program.unwrap();
        Ok(())
    }

    pub fn get_start_point(&self) -> Vec3 {
        Vec3::new(self.vertices[0], self.vertices[1], self.vertices[2])
    }
    pub fn get_end_point(&self) -> Vec3 {
        Vec3::new(self.vertices[3], self.vertices[4], self.vertices[5])
    }
}

impl GraphicDrawable for DrawableLine {
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
            gl.draw_arrays(glow::LINES, 0, 2);

            gl.bind_vertex_array(None);
        };
    }

    fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo);
        };
    }
}
