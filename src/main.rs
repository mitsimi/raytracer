use eframe::egui;
use egui::{Align2, Color32, Vec2};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 540.0]),
        //multisampling: 1 << 0,
        ..Default::default()
    };
    eframe::run_native(
        "Raytracer",
        options,
        Box::new(|_cc| Ok(Box::<RayTracingApp>::default())),
    )
}

#[derive(Default, Clone)]
struct RayTracingApp {
    viewport_height: usize,
    viewport_width: usize,
    texture: Option<egui::TextureHandle>,
}

impl RayTracingApp {
    fn render(&self) -> Vec<Color32> {
        let mut pixels = Vec::with_capacity(self.viewport_height * self.viewport_width);
        for y in 0..self.viewport_height {
            for x in 0..self.viewport_width {
                pixels.push(Color32::from_rgb(
                    (255 * x / self.viewport_width) as u8,
                    (255 * y / self.viewport_height) as u8,
                    0,
                ));
            }
        }
        pixels
    }

    fn update_viewport_size(&mut self, width: f32, height: f32) {
        self.viewport_width = width as usize;
        self.viewport_height = height as usize;
    }
}

impl eframe::App for RayTracingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Settings")
            .anchor(Align2::RIGHT_TOP, Vec2::new(-5.0, 5.0))
            .resizable(false)
            .movable(false)
            .show(ctx, |ui| {
                if ui
                    .add_sized([200.0, 40.0], egui::Button::new("Render"))
                    .clicked()
                {
                    let pixels = self.render();
                    let image =
                        egui::ColorImage::new([self.viewport_width, self.viewport_height], pixels);
                    let texture = self.texture.get_or_insert_with(|| {
                        ctx.load_texture("render", image.clone(), egui::TextureOptions::NEAREST)
                    });
                    texture.set(image, egui::TextureOptions::NEAREST);
                    self.texture = Some(texture.clone());
                }
            });

        egui::Window::new("Render Output")
            .default_size(Vec2::new(480.0, 270.0))
            .title_bar(false)
            .resizable(true)
            .collapsible(false)
            .scroll(true)
            .show(ctx, |ui| {
                self.update_viewport_size(ui.available_width(), ui.available_height());

                if let Some(texture) = &self.texture {
                    let [width, height] = texture.size().map(|x| x as f32);
                    let size = egui::vec2(width, height);
                    ui.image((texture.id(), size));

                    ui.add_space(ui.available_height());
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.label("Click Render to start.");
                    });
                }
            });

        egui::CentralPanel::default().show(ctx, |_ui| {});
    }
}
