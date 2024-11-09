use crate::vm::VirtualMachine;
use notan::prelude::*;

#[derive(AppState)]
pub struct State {
    pub can_update_frame: bool,
    pub frame_timer: f32,
    pub cycle_timer: f32,
    pub cycles_per_frame: u16,
    pub vm: VirtualMachine,
}

pub fn setup(_gfx: &mut Graphics) -> State {
    State {
        can_update_frame: false,
        frame_timer: 0.0,
        cycle_timer: 0.0,
        cycles_per_frame: 10,
        vm: VirtualMachine::new(),
    }
}

pub fn update(app: &mut App, state: &mut State) {
    if state.vm.is_running {
        let delta = app.timer.delta_f32();
        let frame_time = 1.0 / app.timer.fps().min(60.0);
        let cycle_time = frame_time / (state.cycles_per_frame as f32);

        state.cycle_timer += delta;
        state.frame_timer += delta;

        if state.cycle_timer >= cycle_time {
            state.cycle_timer = 0.0;
            state.vm.fde_cycle();
        }

        if state.frame_timer >= frame_time {
            state.frame_timer = 0.0;
            state.can_update_frame = true;
        }
    }
}
