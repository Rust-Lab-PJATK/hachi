use notan::egui::{Align, Grid, Layout, ScrollArea, Ui};
use crate::ui::state::State;

pub fn display_memory_contents(
    ui: &mut Ui,
    state: &mut State,
) {
    ui.label("Memory");

    ui.add_space(10.0);

    ui.with_layout(Layout::left_to_right(Align::Max).with_cross_justify(true), |ui| {
        ScrollArea::vertical()
            .auto_shrink([false, false])
            .max_width(ui.available_size().x)
            .max_height(ui.available_size().y)
            .show(ui, |ui| {
                Grid::new("memory")
                    .striped(true)
                    .min_col_width(200.0)
                    .num_columns(2)
                    .show(ui, |ui| {
                        for (index, byte) in state.vm.memory.iter().enumerate().take(1000) {
                            ui.label(format!("0x{:04X}", index));

                            ui.label(format!("{:02X}", byte));

                            ui.end_row();
                        }
                    });
            });
    });

}