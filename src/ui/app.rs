use std::sync::{Arc, Mutex};

use glam::Vec2;

use crate::ui::graphic::graphic::{GraphicRenderer, GraphicUpdateOptions};

pub struct CalcApp {
    fps: u64,
    graphic_renderer: Arc<Mutex<GraphicRenderer>>,
    pub info: Arc<Mutex<Result<Option<String>, String>>>,
    info_frame_color: Option<egui::Color32>,
}

pub fn create_ui() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 320.0]),
        // multisampling: 1,
        depth_buffer: 24,
        // stencil_buffer: 8,
        multisampling: 2,
        renderer: eframe::Renderer::Glow,
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        ..Default::default()
    };
    eframe::run_native(
        "Rs Calc",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(CalcApp::default(cc).expect("App Couldn't Create")))
        }),
    )
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "msyh".to_owned(),
        egui::FontData::from_static(include_bytes!("../../static/msyh.ttc")).into(),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "msyh".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("msyh".to_owned());

    ctx.set_fonts(fonts);
}

impl CalcApp {
    pub fn default<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
        setup_fonts(&cc.egui_ctx);
        // cc.egui_ctx.set_visuals(egui::Visuals::light());
        Some(Self {
            fps: 120,
            graphic_renderer: Arc::new(Mutex::new(
                GraphicRenderer::default(cc).expect("Unable to create renderer"),
            )),
            info: Arc::new(Mutex::new(Ok(None))),
            info_frame_color: None,
        })
    }

    fn draw_graphic(&self, ui: &mut egui::Ui) {
        let available_size = ui.available_size();
        let desired_size = egui::Vec2::new(available_size.x, available_size.y);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::all());

        // Will Use
        let drag_motion = response.drag_motion();
        let drag_button: Option<egui::PointerButton> =
            if response.dragged_by(egui::PointerButton::Primary) {
                Some(egui::PointerButton::Primary)
            } else if response.dragged_by(egui::PointerButton::Middle) {
                Some(egui::PointerButton::Middle)
            } else if response.dragged_by(egui::PointerButton::Secondary) {
                Some(egui::PointerButton::Secondary)
            } else {
                None
            };

        let graphic_renderer = self.graphic_renderer.clone();
        let graphic_options = GraphicUpdateOptions {
            drag_motion: Vec2::new(drag_motion.x, drag_motion.y),
            drag_button,
        };
        let gl_cb = egui_glow::CallbackFn::new(move |_info, painter| {
            if let Ok(mut graphic_renderer) = graphic_renderer.lock() {
                graphic_renderer.camera.aspect_ratio = desired_size.x / desired_size.y;
                graphic_renderer.paint(painter.gl(), graphic_options.clone());
            }
        });
        let paint_cb = egui::PaintCallback {
            callback: Arc::new(gl_cb),
            rect,
        };
        ui.painter().add(paint_cb);
    }
}

impl eframe::App for CalcApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar_panel").show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Top Pannel");
            });
        });
        egui::TopBottomPanel::bottom("bottom_info_bar_panel")
            .frame({
                let mut frame = egui::Frame::new();
                if let Some(color) = self.info_frame_color {
                    frame = frame.fill(color);
                }
                frame
            })
            .show(ctx, |ui| {
                self.info_frame_color = self.draw_info_ui(ctx, frame, ui);
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .inner_margin(egui::Margin::ZERO)
                    .outer_margin(egui::Margin::ZERO),
            )
            .show(ctx, |ui| {
                egui::Frame::canvas(ui.style()).show(ui, |ui| self.draw_graphic(ui));
                ctx.request_repaint_after(std::time::Duration::from_millis(1000 / self.fps));
            });
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        println!("Quit App");
        if let Some(gl) = gl {
            self.graphic_renderer.lock().unwrap().destroy(gl);
        }
    }
}
