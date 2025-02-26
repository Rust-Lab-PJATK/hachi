use super::state::State;
use notan::app::App;
use notan::egui::{self, CentralPanel, Context, Grid, SidePanel, TopBottomPanel, Ui};
use pollster::FutureExt;
use rfd::AsyncFileDialog;
use std::sync::Arc;
use std::thread;
use crate::ui::debug::{display_memory_contents, vm_display};

pub fn init<'a>(
    _app: &'a mut App,
    state: &'a mut State,
) -> impl FnOnce(&Context) + 'a {
    |ctx| {
        TopBottomPanel::top("toolbar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", file_menu_handler(state));
                ui.menu_button("View", view_menu_handler(state));
            });
        });

        if !state.vm.is_running && !state.debug_mode_enabled {
            CentralPanel::default().show(ctx, |ui| {
                ui.label("Welcome to Hachi!");
            });
        }

        if state.debug_mode_enabled {
            SidePanel::left("output").resizable(false).show(ctx, |ui| {
                ui.label("Output");
            });

            SidePanel::right("history").resizable(false).show(ctx, |ui| {
                ui.label("History");
            });

            CentralPanel::default().show(ctx, |ui| {
                Grid::new("controls")
                    .striped(false)
                    .min_col_width(50.0)
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label("Keypad");
                            ui.label(
                                " 1 2 3 c \n 4 5 6 d \n 7 8 9 e \n a 0 b f",
                            );

                            ui.separator();

                            ui.add_space(525.0);
                        });

                        ui.vertical(|ui| {
                            vm_display(state, ui);

                            ui.separator();

                            display_memory_contents(ui, state);
                        });
                    });
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

fn view_menu_handler(state: &mut State) -> impl FnOnce(&mut Ui) + '_ {
    |ui| {
        ui.checkbox(&mut state.debug_mode_enabled, "Debug mode");
    }
}
