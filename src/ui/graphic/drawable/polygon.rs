use glam::{Mat4, Vec3};
use glow::HasContext;

use crate::ui::graphic::{
    drawable::drawable::GraphicDrawable,
    graphic::GraphicMVPMatrix,
    program::{PROGRAM_MANAGER, ProgramId},
};

pub struct DrawablePolygon {
    color: [f32; 4],
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    ebo: glow::Buffer,
    pub vertices: Vec<Vec3>,

    ind_count: i32,

    program: glow::NativeProgram,
}

impl DrawablePolygon {
    pub fn new(gl: &glow::Context) -> Self {
        unsafe {
            let vao = gl
                .create_vertex_array()
                .expect("Unable to create vertex array");

            let vbo = gl.create_buffer().expect("Unable to create buffer");
            let ebo = gl.create_buffer().expect("Unable to create buffer");

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(0);

            gl.bind_vertex_array(None);

            Self {
                color: [0.0f32; 4],
                vao,
                vbo,
                ebo,
                vertices: Vec::new(),
                ind_count: 0,
                program: PROGRAM_MANAGER
                    .get_program(gl, ProgramId::Default)
                    .expect("Default program not created"),
            }
        }
    }

    pub fn set_verts(&mut self, gl: &glow::Context, verts: &Vec<Vec3>) {
        self.vertices = verts.clone();
        unsafe {
            gl.bind_vertex_array(Some(self.vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            let vert_f32: Vec<f32> = self.vertices.iter().flat_map(|s| [s.x, s.y, s.z]).collect();
            let u8_buffer: &[u8] = bytemuck::cast_slice(&vert_f32[..]);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);

            let mut indices = Vec::<i32>::new();
            self.ind_count = 0;
            if self.vertices.len() >= 3 {
                let mut ptr_inc = 0;
                let mut ptr_dec = (self.vertices.len() - 1) as i32;

                // Won't be unused!!!
                #[allow(unused)]
                let mut ptr_mov = 0;

                loop {
                    ptr_mov = ptr_inc + 1;
                    if ptr_mov == ptr_dec {
                        break;
                    }
                    indices.push(ptr_inc);
                    indices.push(ptr_mov);
                    indices.push(ptr_dec);
                    self.ind_count += 1;
                    ptr_inc += 1;

                    ptr_mov = ptr_dec - 1;
                    if ptr_mov == ptr_inc {
                        break;
                    }
                    indices.push(ptr_inc);
                    indices.push(ptr_mov);
                    indices.push(ptr_dec);
                    self.ind_count += 1;
                    ptr_dec -= 1;
                }
            }

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            let u8_buffer = bytemuck::cast_slice(&indices[..]);
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);

            gl.bind_vertex_array(None);
        }
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
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

impl GraphicDrawable for DrawablePolygon {
    fn draw(&self, gl: &glow::Context, camera: &crate::ui::graphic::camera::GraphicCamera) {
        unsafe {
            gl.use_program(Some(self.program));
            let mvp_transform = GraphicMVPMatrix::from_camera(camera, Mat4::IDENTITY);
            mvp_transform.assign_gl_program(gl, self.program);

            let color_location = gl.get_uniform_location(self.program, "color");
            gl.uniform_4_f32_slice(color_location.as_ref(), &self.color);

            gl.bind_vertex_array(Some(self.vao));
            gl.depth_func(glow::LESS);
            gl.draw_elements(glow::TRIANGLES, self.ind_count * 3, glow::UNSIGNED_INT, 0);

            gl.bind_vertex_array(None);
        }
    }
    fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo);
            gl.delete_buffer(self.ebo);
        };
    }
}
