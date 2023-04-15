use eframe::egui;
use egui_extras::RetainedImage;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 900.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Show an image with eframe/egui",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    name: String,
    value: f32,
    image: RetainedImage,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "World".to_owned(),
            value: 0.5,
            image: RetainedImage::from_image_bytes(
                "rust-logo-256x256.png",
                include_bytes!("../img/Stocking/shime1.png"),
            )
            .unwrap(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.label("This is some non-interactive text.");
            self.image.show(ui);

            if ui.button("Click me").clicked() {
                println!("Hello {}!", self.name);
                println!("Hello {}!", self.value);
            }
        });
    }
}
