use crate::ui::graphic::camera::GraphicCamera;

pub trait GraphicDrawable {
    fn draw(&self, gl: &glow::Context, camera: &GraphicCamera);
    fn destroy(&self, gl: &glow::Context);
}
