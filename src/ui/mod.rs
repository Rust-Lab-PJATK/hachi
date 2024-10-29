use notan::app::{App, Color, Graphics, Plugins};
use notan::egui::{CentralPanel, EguiPluginSugar, TopBottomPanel};
use notan_extra::FpsLimit;

pub fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins) {
    plugins.add(FpsLimit::new(60));

    let emu_display_width: u32 = 640;
    let emu_display_height: u32 = 320;
    let mut toolbar_height: u32 = 0;

    let mut ui_renderer = plugins.egui(|ctx| {
        let toolbar = TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("File");
            });
        });
        toolbar_height = toolbar.response.rect.height().ceil() as u32;

        CentralPanel::default().show(ctx, |ui| {
            ui.label("Welcome to Hachi!");
        });
    });

    app.window()
        .set_size(emu_display_width, emu_display_height + toolbar_height);

    ui_renderer.clear_color(Color::BLACK);
    gfx.render(&ui_renderer);
}
