use eframe::Frame;
#[allow(unused_imports)]
use egui::{
    Context,
    CentralPanel,
    SidePanel,
    Label,
    ViewportBuilder,
    ViewportCommand,
    TopBottomPanel,
    Visuals,
    menu,
    Window,
};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([600.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Panel Demo",
        options,
        Box::new(|cc| Ok(Box::new(PanelDemo::new(cc))))
    )
}

#[derive(Default)]
struct PanelDemo {
}

impl PanelDemo {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        Default::default()
    }
}

impl eframe::App for PanelDemo {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {

        TopBottomPanel::top("top_panel 0").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        println!("Save Clicked.");
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut").clicked() {
                        println!("Cut Clicked.");
                    }
                    if ui.button("Copy").clicked() {
                        println!("Copy Clicked.");
                    }
                    if ui.button("Paste").clicked() {
                        println!("Paste Clicked.");
                    }
                });
            });
        });

        SidePanel::left("left_panel").show(ctx, |ui| {
            ui.label("Side Panel #1")
        });

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label("Top Panel #2");
        });

        SidePanel::right("right1").show(ctx, |ui| {
            ui.label("Side Panel #2");
        });

        SidePanel::right("right2").show(ctx, |ui| {
            ui.label("Side Panel #3");
        });

        CentralPanel::default().show(ctx, |ui| {
            // Window::new("Window1")
            //     .show(ctx, |ui| {
            //     ui.add(Label::new("A small window #1"));
            // });
            //
            // Window::new("Window2")
            //     .constrain(true)
            //     .show(ctx, |ui| {
            //     ui.add(Label::new("A small window #2"));
            // });
        });
    }
}