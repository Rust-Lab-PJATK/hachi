use super::state::State;
use notan::app::App;
use notan::egui::{self, CentralPanel, Context, TopBottomPanel, Ui};
use pollster::FutureExt;
use rfd::AsyncFileDialog;
use std::sync::Arc;
use std::thread;

pub fn init<'a>(
    app: &'a mut App,
    state: &'a mut State,
) -> impl FnOnce(&Context) + 'a {
    |ctx| {
        TopBottomPanel::top("toolbar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", file_menu_handler(state));
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
    }
}

fn file_menu_handler(state: &mut State) -> impl FnOnce(&mut Ui) + '_ {
    |ui| {
        if ui.button("Open").clicked() {
            ui.close_menu();

            let last_dir = &state.last_dir;
            let fpo_guard = state.file_path_option.try_lock();

            if fpo_guard.is_some() {
                drop(fpo_guard);

                let fpo_arc = Arc::clone(&state.file_path_option);
                let file_dialog_future = AsyncFileDialog::new()
                    .add_filter("Program File (*.ch8)", &["ch8"])
                    .set_directory(last_dir)
                    .pick_file();

                thread::spawn(move || {
                    async {
                        let mut file_path_option = fpo_arc.lock().await;

                        if let Some(file_handle) = file_dialog_future.await {
                            *file_path_option =
                                Some(file_handle.path().to_path_buf())
                        }
                    }
                    .block_on()
                });
            }
        }
    }
}
