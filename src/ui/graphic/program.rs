use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use glow::{HasContext, NativeProgram};
use once_cell::sync::Lazy;

pub fn compile_shader_program(
    gl: &glow::Context,
    program: &glow::NativeProgram,
    vert: &str,
    frag: &str,
) {
    unsafe {
        let vert_shader = gl
            .create_shader(glow::VERTEX_SHADER)
            .expect("Unable to create vertex shader");
        gl.shader_source(vert_shader, vert);
        gl.compile_shader(vert_shader);

        if !gl.get_shader_compile_status(vert_shader) {
            panic!(
                "Vert shader compile failed:\n{}",
                gl.get_shader_info_log(vert_shader)
            );
        }

        let frag_shader = gl
            .create_shader(glow::FRAGMENT_SHADER)
            .expect("Unable to create fragment shader");
        gl.shader_source(frag_shader, frag);
        gl.compile_shader(frag_shader);

        if !gl.get_shader_compile_status(frag_shader) {
            panic!(
                "Frag shader compile failed:\n{}",
                gl.get_shader_info_log(frag_shader)
            );
        }

        gl.attach_shader(*program, vert_shader);
        gl.attach_shader(*program, frag_shader);
        gl.link_program(*program);

        gl.detach_shader(*program, vert_shader);
        gl.detach_shader(*program, frag_shader);
        gl.delete_shader(vert_shader);
        gl.delete_shader(frag_shader);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ProgramId {
    Default,
    DrawableLine,
    DrawableArrow,
}

impl std::fmt::Display for ProgramId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({})",
            match self {
                ProgramId::Default => "Default",
                ProgramId::DrawableLine => "Drawable Line",
                ProgramId::DrawableArrow => "Drawable Arrow",
            }
        )
    }
}

#[derive(Clone)]
enum ManagedProgram {
    RAW {
        vert_shader: &'static str,
        frag_shader: &'static str,
    },
    COMPILED(glow::NativeProgram),
}

#[derive(Clone)]
pub struct ProgramManager {
    programs: Arc<RwLock<HashMap<ProgramId, ManagedProgram>>>,
}

impl ProgramManager {
    fn new() -> Self {
        let mut programs = HashMap::new();

        programs.insert(
            ProgramId::Default,
            ManagedProgram::RAW {
                vert_shader: include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/static/shader/vert.hlsl"
                )),
                frag_shader: include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/static/shader/frag.hlsl"
                )),
            },
        );
        programs.insert(
            ProgramId::DrawableLine,
            ManagedProgram::RAW {
                vert_shader: include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/static/shader/drawable/line_vert.hlsl"
                )),
                frag_shader: include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/static/shader/drawable/line_frag.hlsl"
                )),
            },
        );

        Self {
            programs: Arc::new(RwLock::new(programs)),
        }
    }
    pub fn get_program(&self, gl: &glow::Context, id: ProgramId) -> Option<NativeProgram> {
        let binding = self.programs.read().unwrap();
        let managed_program = binding.get(&id);
        if managed_program.is_none() {
            return None;
        }
        match managed_program.unwrap() {
            ManagedProgram::RAW {
                vert_shader,
                frag_shader,
            } => unsafe {
                println!("Compiling for program {}", id);
                let program = gl.create_program().expect("Unable to create program");
                compile_shader_program(gl, &program, vert_shader, frag_shader);

                let _ = managed_program;
                drop(binding);
                let mut writable_programs = self.programs.write().unwrap();
                writable_programs.insert(id, ManagedProgram::COMPILED(program));
                Some(program.clone())
            },
            ManagedProgram::COMPILED(program) => Some(program.clone()),
        }
    }

    pub fn delete_all_program(&self, gl: &glow::Context) {
        let mut program_map = self.programs.write().unwrap();
        for (id, val) in program_map.iter_mut() {
            if let ManagedProgram::COMPILED(program) = val {
                unsafe {
                    gl.delete_program(*program);
                }
                println!("Delete program {}.", id);
            }
        }
    }
}

pub static PROGRAM_MANAGER: Lazy<ProgramManager> = Lazy::new(ProgramManager::new);
