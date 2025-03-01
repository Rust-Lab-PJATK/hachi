use crate::ui::consts::MEMORY_MAX_DISPLAY_ITEMS;
use crate::ui::state::State;
use crate::vm::consts::{DEBUG_DISPLAY_HEIGHT, DEBUG_DISPLAY_WIDTH};
use notan::egui;
use notan::egui::{
    vec2, Align, Button, Color32, Frame, Grid, Layout, Mesh, Rect, ScrollArea,
    Shape, TextEdit, Ui, Vec2,
};

fn filter_by_memory_address(index: &usize, state: &State) -> bool {
    if state.memory_address_search.is_empty() {
        return true;
    }

    let search_address =
        usize::from_str_radix(&state.memory_address_search, 16).unwrap_or(0);

    *index == search_address
}

pub fn vm_display(state: &mut State, ui: &mut Ui) {
    Frame::canvas(ui.style()).show(ui, |ui| {
        let rect = ui.max_rect();
        let desired_size =
            Vec2::new(DEBUG_DISPLAY_WIDTH as f32, DEBUG_DISPLAY_HEIGHT as f32);

        let mut vm_display_mesh = Mesh::with_texture(state.vm_display.id);
        vm_display_mesh.add_rect_with_uv(
            Rect::from_min_size(rect.min, desired_size),
            Rect::from_min_max(egui::pos2(0.0, 1.0), egui::pos2(1.0, 0.0)),
            Color32::WHITE,
        );

        ui.painter().add(Shape::mesh(vm_display_mesh));
    });

    ui.add_space(270.0);
}

pub fn display_memory_contents(ui: &mut Ui, state: &mut State) {
    ui.horizontal(|ui| {
        ui.label("Memory");

        if ui.add(Button::new("<").min_size(vec2(0.0, 14.0))).clicked() {
            if state.memory_debug_page == 0 {
                return;
            }

            state.memory_debug_page -= 1;
        } else if ui.add(Button::new(">").min_size(vec2(0.0, 14.0))).clicked() {
            if (state.memory_debug_page + 1) * MEMORY_MAX_DISPLAY_ITEMS
                > state.vm.memory.len()
            {
                return;
            }

            state.memory_debug_page += 1;
        }

        ui.add(
            TextEdit::singleline(&mut state.memory_address_search)
                .desired_width(125.0)
                .hint_text("Find memory address"),
        );
    });

    ui.add_space(4.0);

    ui.with_layout(
        Layout::left_to_right(Align::Max).with_cross_justify(true),
        |ui| {
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_width(ui.available_size().x)
                .max_height(225.0)
                .show(ui, |ui| {
                    Grid::new("memory")
                        .striped(true)
                        .min_col_width(200.0)
                        .num_columns(2)
                        .show(ui, |ui| {
                            for (index, byte) in state
                                .vm
                                .memory
                                .iter()
                                .enumerate()
                                .skip(
                                    state.memory_debug_page
                                        * MEMORY_MAX_DISPLAY_ITEMS,
                                )
                                .take(MEMORY_MAX_DISPLAY_ITEMS)
                                .filter(|(index, _)| {
                                    filter_by_memory_address(index, state)
                                })
                            {
                                ui.label(format!("0x{:04X}", index));

                                ui.label(format!("{:02X}", byte));

                                ui.end_row();
                            }
                        });
                });
        },
    );
}
