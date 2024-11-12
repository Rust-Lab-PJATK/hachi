use super::state::State;
use crate::vm::consts::DISPLAY_WIDTH;
use notan::app::{App, Color, Graphics, Plugins};
use notan::draw::{CreateDraw, DrawShapes};
use notan::egui::{
    self, CentralPanel, Context, EguiPluginSugar, TopBottomPanel,
};
use pollster::FutureExt as _;
use rfd::AsyncFileDialog;
use std::sync::Arc;
use std::thread;

pub fn draw(
    app: &mut App,
    gfx: &mut Graphics,
    plugins: &mut Plugins,
    state: &mut State,
) {
    let mut ui_renderer = plugins.egui(|ctx: &Context| {
        TopBottomPanel::top("toolbar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        ui.close_menu();

                        let last_dir = &state.last_dir;
                        let fho_guard = state.file_handle_option.try_lock();

                        if fho_guard.is_some() {
                            drop(fho_guard);

                            let fho_arc = Arc::clone(&state.file_handle_option);
                            let file_handle_future = AsyncFileDialog::new()
                                .add_filter("Program File (*.ch8)", &["ch8"])
                                .set_directory(last_dir)
                                .pick_file();

                            thread::spawn(move || {
                                async {
                                    let mut file_handle_option =
                                        fho_arc.lock().await;

                                    *file_handle_option =
                                        file_handle_future.await;
                                }
                                .block_on()
                            });
                        }
                    }
                });
            });
        });

        TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("FPS: {}", app.timer.fps().round()));
            });
        });

        if !state.vm.is_running {
            CentralPanel::default().show(ctx, |ui| {
                ui.label("Welcome to Hachi!");
            });
        }
    });

    let mut vm_display_renderer = gfx.create_draw();

    if state.vm.is_running {
        let display_width = app.window().width() as f32;
        let display_height = display_width / 2.0;
        let rect_width = display_width / DISPLAY_WIDTH as f32;
        let colors: [Color; 2] = [
            Color::from_hex(0xA7C9A1FF), // PIXEL OFF
            Color::from_hex(0x282D2DFF), // PIXEL ON
        ];
        let pixel_rows = state.vm.video_memory.into_iter().enumerate();

        for (row_index, pixel_row) in pixel_rows {
            let pixels = pixel_row.iter().enumerate();

            for (column_index, pixel) in pixels {
                let x_position = column_index as f32 * rect_width;
                let y_offset =
                    (app.window().height() as f32 - display_height) / 2.0;
                let y_position = y_offset + (row_index as f32 * rect_width);

                vm_display_renderer
                    .rect((x_position, y_position), (rect_width, rect_width))
                    .fill_color(colors[*pixel as usize]);
            }
        }

        ui_renderer.clear_color(Color::BLACK);
    }

    gfx.render(&ui_renderer);
    gfx.render(&vm_display_renderer);
}
