use crate::vm::VirtualMachine;
use notan::prelude::*;

#[derive(AppState)]
pub struct State {
    pub frame_timer: f32,
    pub cycle_timer: f32,
    pub cycles_per_frame: u16,
    pub vm: VirtualMachine,
}

pub const FRAME_TIME: f32 = 1.0 / 60.0;

pub fn setup(_gfx: &mut Graphics) -> State {
    State {
        frame_timer: 0.0,
        cycle_timer: 0.0,
        cycles_per_frame: 10,
        vm: VirtualMachine::new(),
    }
}

pub fn update(app: &mut App, state: &mut State) {
    if state.vm.is_running {
        let delta = app.timer.delta_f32();
        let cycle_time = FRAME_TIME / (state.cycles_per_frame as f32);

        state.cycle_timer += delta;
        state.frame_timer += delta;

        if state.cycle_timer >= cycle_time {
            state.cycle_timer = 0.0;
            state.vm.fde_cycle();
        }
    }
}
