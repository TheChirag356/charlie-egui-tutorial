// modules
use egui::{Context, CentralPanel, Label, SidePanel, };

fn main() -> Result<(), eframe::Error>{
    eframe::run_native(
        "MyApp",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MyApp::new(cc))))
    )
}

#[derive(Default)]
struct MyApp {}

impl MyApp  {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.label("Hello, World!");
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.label("ui.label");
            ui.add(Label::new("ui.add"));
            ui.heading("ui.heading");
        });
    }
}