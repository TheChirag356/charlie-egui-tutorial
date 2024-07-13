use eframe::{Frame, };
use egui::{Context, CentralPanel, Button, Widget, Ui, };

fn main() -> Result<(), eframe::Error>{
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "Button Demo",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc))))
    )
}

#[derive(Default)]
struct MyApp {
    counter: i32,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {

            if ui.add(Button::new("Click Me")).clicked() {
                println!("Click Me");
            }

            if ui.small_button("0").clicked() {
                println!("0");
            }
            if Button::new("print 4").ui(ui).clicked() {
                println!("4");
            }
            if ui.add_enabled(false, Button::new("Can't click this")).clicked() {
                unreachable!();
            }
            if ui.button("Quit").clicked() {
                let ctx = ctx.clone();
                std::thread::spawn(move || {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                });
            };
        });
    }
}