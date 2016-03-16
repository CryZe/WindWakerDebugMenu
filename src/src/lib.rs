#![no_std]
#![feature(asm, const_fn)]

#[macro_use]
extern crate libtww;

use libtww::system;
use libtww::game::{controller, Console};

pub mod flag_editor;
pub mod main_menu;
pub mod warp_menu;
pub mod flag_menu;
pub mod inventory_menu;
pub mod cheat_menu;
pub mod utils;
pub mod popups;

use utils::*;

pub static mut cursor: usize = 0;
pub static mut visible: bool = false;

#[no_mangle]
#[inline(never)]
pub extern "C" fn init() {
    // Call overriden instruction
    system::cdyl_init_async();

    let mut console = Console::get();
    console.line_count = 32;
    console.x = 0;
    console.y = 16;
    console.font_scale_x *= 1.2;
    console.font_scale_y *= 1.2;
    console.background_color.a = 150;
    console.clear();
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn game_loop() {
    cheat_menu::apply_cheats();

    if unsafe { visible } {
        match unsafe { menu_state } {
            MenuState::MainMenu => main_menu::render(),
            MenuState::WarpMenu => warp_menu::render(),
            MenuState::FlagMenu => flag_menu::render(),
            MenuState::InventoryMenu => inventory_menu::render(),
            MenuState::CheatMenu => cheat_menu::render(),
        }
    // Prevent opening the Debug Menu if in map select and map select cheat is inactive
    } else if system::memory::read::<u8>(0x803BD254) == 0x01 && system::memory::read::<u32>(0x80AF03E4) == 0x8038824C { 
        //Do nothing
    } else if is_pressed(controller::DPAD_DOWN) && unsafe { !popups::visible } {
        let console = Console::get();
        console.visible = true;
        unsafe { visible = true; }
    } else {
        // Only check popups if the Debug Menu is not open
        popups::check_global_flags();
    }
}

#[no_mangle]
pub extern "C" fn start() {
    game_loop();
    init();
    let _ = read_controller();
}
