fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "frame per second",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    num: usize,
    current_time: std::time::Instant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            num: 0,
            current_time: std::time::Instant::now(),
        }
    }
}

// update the num per second
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = std::time::Instant::now();
        if now.duration_since(self.current_time).as_secs_f32() > 1.0 {
            self.current_time = now;
            self.num += 1;
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("frame per second: {}", self.num));
        });
    }
}
