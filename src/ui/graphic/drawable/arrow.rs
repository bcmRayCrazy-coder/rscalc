// use glam::Vec3;
// use glow::HasContext;

// use crate::ui::graphic::{
//     drawable::{drawable::GraphicDrawable, line::DrawableLine},
//     program::{PROGRAM_MANAGER, ProgramId},
// };

// #[derive(Debug, Clone)]
// pub struct DrawableArrow {
//     program: glow::NativeProgram,
// }

// impl DrawableArrow {
//     pub fn new(gl: &glow::Context) -> Self {
//         Self {
//             program: PROGRAM_MANAGER
//                 .get_program(gl, ProgramId::DrawableArrow)
//                 .expect("Drawable Arrow program not created"),
//         }
//     }

//     pub fn set_points(&mut self, gl: &glow::Context, start: Vec3, end: Vec3) {
//         // let rev_normal = (start - end).normalize();
//         // rev_normal.ro
//     }

//     pub fn set_color(&mut self, color: [f32; 4]) {}

//     pub fn set_line_width(&mut self, line_width: f32) {}

//     pub fn set_program(&mut self, gl: &glow::Context, id: &ProgramId) -> Result<(), String> {
//         let program = PROGRAM_MANAGER.get_program(gl, id.clone());
//         if program.is_none() {
//             return Err(format!("Program {} is None.", id));
//         }
//         self.program = program.unwrap();
//         Ok(())
//     }
// }

// impl GraphicDrawable for DrawableArrow {
//     fn draw(&self, gl: &glow::Context, camera: &crate::ui::graphic::camera::GraphicCamera) {
//         todo!()
//     }
// }
