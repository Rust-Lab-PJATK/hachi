use crate::vm::consts::{DEBUG_DISPLAY_HEIGHT, DEBUG_DISPLAY_WIDTH};
use crate::vm::VirtualMachine;
use async_mutex::Mutex as AsyncMutex;
use clap::Parser;
use notan::egui::{EguiRegisterTexture, SizedTexture};
use notan::prelude::*;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::Arc;
use crate::config::Configuration;

#[derive(AppState)]
pub struct State {
    pub last_dir: PathBuf,
    pub configuration: Configuration,
    pub file_path_option: Arc<AsyncMutex<Option<PathBuf>>>,
    pub cycles_per_frame: u32,
    pub memory_debug_page: usize,
    pub memory_address_search: String,
    pub vm: VirtualMachine,
    pub debug_mode_enabled: bool,
    pub display_renderer: RenderTexture,
    pub vm_display: SizedTexture,
    pub keypad_bindigs: HashMap<KeyCode, usize>,
    pub timer: f32,
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_os_t)]
    file: OsString,
}

pub fn setup(gfx: &mut Graphics) -> State {
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

    let display_renderer = gfx
        .create_render_texture(DEBUG_DISPLAY_WIDTH, DEBUG_DISPLAY_HEIGHT)
        .build()
        .unwrap();

    let vm_display_texture = gfx.egui_register_texture(&display_renderer);

    // TODO: move these bindings into config
    // Going left-to-right, row-by-row
    // 1 2 3 4  ->  1 2 3 C
    // Q W E R  ->  4 5 6 D
    // A S D F  ->  7 8 9 E
    // Z X C V  ->  A 0 B F
    let default_bindings = [
        (KeyCode::Key1, 0x1),
        (KeyCode::Key2, 0x2),
        (KeyCode::Key3, 0x3),
        (KeyCode::Key4, 0xC),
        (KeyCode::Q, 0x4),
        (KeyCode::W, 0x5),
        (KeyCode::E, 0x6),
        (KeyCode::R, 0xD),
        (KeyCode::A, 0x7),
        (KeyCode::S, 0x8),
        (KeyCode::D, 0x9),
        (KeyCode::F, 0xE),
        (KeyCode::Z, 0xA),
        (KeyCode::X, 0x0),
        (KeyCode::C, 0xB),
        (KeyCode::V, 0xF),
    ];

    let config = Configuration::read().unwrap_or_else(|_| {
        let c = Configuration::new();
        c.update();

        c
    });

    State {
        last_dir,
        configuration: config,
        file_path_option: Arc::new(AsyncMutex::new(file_path)),
        cycles_per_frame: 10,
        memory_debug_page: 0,
        memory_address_search: String::new(),
        vm: VirtualMachine::new(),
        debug_mode_enabled: false,
        display_renderer,
        vm_display: vm_display_texture,
        keypad_bindigs: default_bindings.into(),
        timer: 0.0,
    }
}

pub fn update(app: &mut App, state: &mut State) {
    let fpo_guard = state.file_path_option.try_lock();

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

    // Timer executed at a rate of 60hz
    state.timer += app.timer.delta_f32();
    while state.timer >= 1.0 / 60.0 {
        state.timer -= 1.0 / 60.0;
        if state.vm.delay_timer != 0 {
            state.vm.delay_timer -= 1;
        }
        if state.vm.sound_timer != 0 {
            // TODO: play sound here
            state.vm.sound_timer -= 1;
        }
    }

    // Update keypad down-keys
    for (key, value) in &state.keypad_bindigs {
        state.vm.keypad[*value] = app.keyboard.is_down(*key);
    }

    // Handle FX0A GETKEY instruction if it currently halts
    if state.vm.is_waiting_for_key {
        for (key, value) in &state.keypad_bindigs {
            if app.keyboard.was_released(*key) {
                state.vm.current_key_press = Some(*value as u8);
            }
        }
    }

    if state.vm.is_running {
        for _ in 0..state.cycles_per_frame {
            state.vm.fde_cycle();
        }
    }
}
