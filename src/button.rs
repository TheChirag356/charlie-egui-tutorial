use eframe::{Frame};
use eframe::emath::{Vec2, Pos2, };
use egui::{Context, CentralPanel, Button, Widget, Ui, ViewportBuilder, };

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder {
            inner_size: Some(Vec2 { x: 200., y: 200. }),
            position: Some(Pos2 { x: 0., y: 0. }),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Button Demo",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
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
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            in_dec_demo(ui, &mut self.counter, ctx);
        });
    }
}

fn in_dec_demo(ui: &mut Ui, counter: &mut i32, ctx: &Context) {
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
        if ui.button("Quit").clicked() {
            let ctx = ctx.clone();
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });
}

#[allow(dead_code)]
fn button_demo(ui: &mut Ui, ctx: &Context) {
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
}