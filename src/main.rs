mod emu;
mod ui;

use notan::draw::DrawConfig;
use notan::egui::EguiConfig;
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::new()
        .set_title("Hachi")
        .set_vsync(true)
        .set_high_dpi(true);

    notan::init()
        .add_config(win)
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .draw(ui::draw)
        .build()
}
