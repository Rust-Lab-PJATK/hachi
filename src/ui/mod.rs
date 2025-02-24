pub(super) mod egui_plugin;
pub mod state;

use crate::vm::consts::DISPLAY_WIDTH;
use notan::app::{App, Color, Graphics, Plugins};
use notan::draw::{CreateDraw, DrawShapes};
use notan::egui::EguiPluginSugar;
use state::State;

pub fn draw(
    app: &mut App,
    gfx: &mut Graphics,
    plugins: &mut Plugins,
    state: &mut State,
) {
    let mut ui_renderer = plugins.egui(egui_plugin::init(app, state));
    let mut vm_display_renderer = gfx.create_draw();

    if state.vm.is_running {
        let vm_display_width = app.window().width() as f32;
        let vm_display_height = vm_display_width / 2.0;
        let rect_width = vm_display_width / DISPLAY_WIDTH as f32;

        let y_offset = if state.debug_mode_enabled {
            0.0
        } else {
            (app.window().height() as f32 - vm_display_height) / 2.0
        };

        let colors: [Color; 2] = [
            Color::from_hex(0xA7C9A1FF), // PIXEL OFF
            Color::from_hex(0x282D2DFF), // PIXEL ON
        ];
        let pixel_rows = state.vm.video_memory.into_iter().enumerate();

        for (row_index, pixel_row) in pixel_rows {
            let pixels = pixel_row.iter().enumerate();

            for (column_index, pixel) in pixels {
                let x_position = column_index as f32 * rect_width;
                let y_position = row_index as f32 * rect_width + y_offset;

                vm_display_renderer
                    .rect((x_position, y_position), (rect_width, rect_width))
                    .fill_color(colors[*pixel as usize]);
            }
        }

        ui_renderer.clear_color(Color::BLACK);
    }

    gfx.render(&ui_renderer);

    if state.debug_mode_enabled {
        gfx.render_to(&state.display_renderer, &vm_display_renderer);
    } else {
        gfx.render(&vm_display_renderer);
    }
}
