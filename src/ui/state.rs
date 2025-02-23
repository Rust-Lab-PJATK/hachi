use crate::vm::consts::{DEBUG_DISPLAY_HEIGHT, DEBUG_DISPLAY_WIDTH};
use crate::vm::VirtualMachine;
use async_mutex::Mutex as AsyncMutex;
use clap::Parser;
use notan::egui::{EguiRegisterTexture, SizedTexture};
use notan::prelude::*;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(AppState)]
pub struct State {
    pub last_dir: PathBuf,
    pub file_path_option: Arc<AsyncMutex<Option<PathBuf>>>,
    pub cycles_per_frame: u32,
    pub vm: VirtualMachine,
    pub debug_mode_enabled: bool,
    pub display_renderer: RenderTexture,
    pub vm_display: SizedTexture,
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
        Some(PathBuf::from(args.file).canonicalize().unwrap())
    };
    let last_dir = if let Some(path) = &file_path {
        path.parent().unwrap().to_path_buf()
    } else {
        std::env::current_dir().unwrap()
    };

    let display_renderer = _gfx
        .create_render_texture(DEBUG_DISPLAY_WIDTH, DEBUG_DISPLAY_HEIGHT)
        .build()
        .unwrap();

    let vm_display_texture = _gfx.egui_register_texture(&display_renderer);

    State {
        last_dir,
        file_path_option: Arc::new(AsyncMutex::new(file_path)),
        cycles_per_frame: 10,
        vm: VirtualMachine::new(),
        debug_mode_enabled: false,
        display_renderer,
        vm_display: vm_display_texture,
    }
}

pub fn update(app: &mut App, state: &mut State) {
    let fpo_guard = state.file_path_option.try_lock();

    // TODO:
    // if app.keyboard != state.vm.keyboard
    //    ustawiamy takie keys down jakie actually są

    if let Some(mut fpo_guard) = fpo_guard {
        if let Some(file_path) = fpo_guard.as_ref() {
            state.last_dir = file_path.parent().unwrap().to_path_buf();

            app.window().set_title(
                format!(
                    "{} - Hachi",
                    file_path.file_name().unwrap().to_string_lossy()
                )
                .as_str(),
            );
            state.vm.reset();
            state.vm.load_program(file_path);

            *fpo_guard = None;
        }
    };

    if state.vm.is_running {
        for _ in 0..state.cycles_per_frame {
            state.vm.fde_cycle();
        }
    }
}
