use crate::vm::VirtualMachine;
use notan::prelude::*;

#[derive(AppState)]
pub struct State {
    pub fde_cycles_per_second: u32,
    pub vm: VirtualMachine,
}

pub fn setup(_gfx: &mut Graphics) -> State {
    State { fde_cycles_per_second: 700, vm: VirtualMachine::new() }
}

pub fn update(app: &mut App, state: &mut State) {
    let fps = app.timer.fps().round() as u32;

    if state.vm.is_running && (1u32..=60).contains(&fps) {
        let num_cycles = state.fde_cycles_per_second / fps;

        for _ in 0..num_cycles {
            state.vm.fde_cycle();
        }
    }
}
