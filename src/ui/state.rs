use crate::vm::VirtualMachine;
use async_mutex::Mutex as AsyncMutex;
use notan::prelude::*;
use rfd::FileHandle;
use std::sync::Arc;

#[derive(AppState)]
pub struct State {
    pub file_handle_option: Arc<AsyncMutex<Option<FileHandle>>>,
    pub cycles_per_second: u32,
    pub vm: VirtualMachine,
}

pub fn setup(_gfx: &mut Graphics) -> State {
    State {
        file_handle_option: Arc::new(AsyncMutex::new(None)),
        cycles_per_second: 700,
        vm: VirtualMachine::new(),
    }
}

pub fn update(app: &mut App, state: &mut State) {
    let fho_guard = state.file_handle_option.try_lock();

    if fho_guard.is_some() {
        let mut fho_guard = fho_guard.unwrap();

        if let Some(file_handle) = fho_guard.as_ref() {
            state.vm.load_program(file_handle.path());
            *fho_guard = None;
            drop(fho_guard);
        }
    };

    if state.vm.is_running {
        let fps = app.timer.fps().round() as u32;
        let num_cycles = state.cycles_per_second.checked_div(fps).unwrap_or(0);

        for _ in 0..num_cycles {
            state.vm.fde_cycle();
        }
    }
}
