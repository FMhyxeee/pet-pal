use egui_extras::RetainedImage;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Native file dialogs and drag-and-drop files",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    images: Vec<RetainedImage>,
    current_image: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut images = Vec::new();
        let files = std::fs::read_dir("img/Stocking").unwrap();
        for file in files {
            let file = file.unwrap();
            let path = file.path();
            let path = path.to_str().unwrap();
            let image = std::fs::read(path).unwrap();
            let image = RetainedImage::from_image_bytes(path, &image).unwrap();
            images.push(image);
        }
        Self {
            images,
            current_image: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.label("This is some non-interactive text.");
            self.images[self.current_image].show(ui);

            if ui.button("Click me").clicked() {
                println!("Hello {}!", self.current_image);
                self.current_image += 1;
                if self.current_image >= self.images.len() {
                    self.current_image = 0;
                }
            }
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let app = MyApp::default();
        assert_eq!(app.images.len(), 43);
        for img in app.images {
            assert_eq!(img.width(), 128);
            assert_eq!(img.height(), 128);
        }
    }
}
