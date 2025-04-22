use crate::ui::state::State;
use notan::egui;
use notan::egui::Window;

pub fn option_dialog(state: &mut State, ctx: &egui::Context) {
    Window::new("Options")
        .max_height(100.0)
        .max_width(200.0)
        .resizable(false)
        .show(ctx, |ui| {
            let mut cycles_per_frame_raw_text =
                state.configuration.vm.cycles_per_frame.to_string();

            ui.horizontal(|ui| {
                ui.label("Cycles per frame");
                if ui
                    .text_edit_singleline(&mut cycles_per_frame_raw_text)
                    .changed()
                {
                    if let Ok(value) = cycles_per_frame_raw_text.parse::<u32>()
                    {
                        state.configuration.vm.cycles_per_frame = value;
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut state.configuration.debug.enable_debug_menu,
                    "Run \"Debug mode\" on start",
                );
            });

            ui.add_space(20.0);

            ui.centered_and_justified(|ui| {
                if ui.button("Close").clicked() {
                    state.configuration.update().unwrap();

                    state.cycles_per_frame =
                        state.configuration.vm.cycles_per_frame;
                    state.show_configuration_window = false;
                }
            });
        });
}
