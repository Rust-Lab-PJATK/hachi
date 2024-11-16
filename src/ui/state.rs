use crate::vm::VirtualMachine;
use async_mutex::Mutex as AsyncMutex;
use clap::Parser;
use notan::prelude::*;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(AppState)]
pub struct State {
    pub last_dir: PathBuf,
    pub file_handle_option: Arc<AsyncMutex<Option<PathBuf>>>,
    pub cycles_per_frame: u32,
    pub vm: VirtualMachine,
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_os_t)]
    file: OsString,
}

pub fn setup(_gfx: &mut Graphics) -> State {
    let args = Args::parse();

    let file_path = if args.file.is_empty() {
        None
    } else {
        Some(PathBuf::from(args.file))
    };

    let last_dir = if let Some(path) = &file_path {
        path.parent().unwrap().to_path_buf()
    } else {
        std::env::current_dir().unwrap()
    };

    State {
        last_dir,
        file_handle_option: Arc::new(AsyncMutex::new(file_path)),
        cycles_per_frame: 10,
        vm: VirtualMachine::new(),
    }
}

pub fn update(_app: &mut App, state: &mut State) {
    let fho_guard = state.file_handle_option.try_lock();

    if let Some(mut fho_guard) = fho_guard {
        if let Some(file_path) = fho_guard.as_ref() {
            state.last_dir = file_path.parent().unwrap().to_path_buf();
            state.vm.reset();
            state.vm.load_program(file_path);
            *fho_guard = None;
        }
    };

    if state.vm.is_running {
        for _ in 0..state.cycles_per_frame {
            state.vm.fde_cycle();
        }
    }
}
