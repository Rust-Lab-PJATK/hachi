mod ui;
mod vm;

use notan::draw::DrawConfig;
use notan::egui::EguiConfig;
use notan::prelude::*;
use notan_extra::FpsLimit;

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::new().set_resizable(false).set_title("Hachi").set_high_dpi(true);

    notan::init_with(ui::state::setup)
        .add_config(win)
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .add_plugin(FpsLimit::new(60))
        .update(ui::state::update)
        .draw(ui::draw)
        .build()
}
