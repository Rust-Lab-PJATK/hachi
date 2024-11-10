use crate::vm::VirtualMachine;
use notan::prelude::*;

#[derive(AppState)]
pub struct State {
    pub instructions_per_second: u32,
    pub vm: VirtualMachine,
}

pub fn setup(_gfx: &mut Graphics) -> State {
    State { instructions_per_second: 700, vm: VirtualMachine::new() }
}

pub fn update(app: &mut App, state: &mut State) {
    if state.vm.is_running {
        let num_cycles = state
            .instructions_per_second
            .checked_div(app.timer.fps().round() as u32)
            .unwrap_or(0);

        for _ in 0..num_cycles {
            state.vm.fde_cycle();
        }
    }
}
