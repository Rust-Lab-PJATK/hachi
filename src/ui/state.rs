use crate::vm::VirtualMachine;
use async_mutex::Mutex as AsyncMutex;
use notan::prelude::*;
use rfd::FileHandle;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(AppState)]
pub struct State {
    pub last_dir: PathBuf,
    pub file_handle_option: Arc<AsyncMutex<Option<FileHandle>>>,
    pub cycles_per_second: u32,
    pub vm: VirtualMachine,
}

pub fn setup(_gfx: &mut Graphics) -> State {
    State {
        last_dir: std::env::current_dir().unwrap(),
        file_handle_option: Arc::new(AsyncMutex::new(None)),
        cycles_per_second: 700,
        vm: VirtualMachine::new(),
    }
}

pub fn update(app: &mut App, state: &mut State) {
    let fho_guard = state.file_handle_option.try_lock();

    if let Some(mut fho_guard) = fho_guard {
        if let Some(file_handle) = fho_guard.as_ref() {
            let file_path = file_handle.path();

            state.last_dir = file_path.parent().unwrap().to_path_buf();
            state.vm.load_program(file_path);
            *fho_guard = None;
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
