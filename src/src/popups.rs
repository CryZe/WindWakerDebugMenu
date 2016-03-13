use libtww::prelude::*;
use libtww::system::memory::read;
use libtww::system::get_frame_count;
use libtww::Addr;
use libtww::game::Console;
use libtww::game::Flag;

use flag_menu::FLAGS;

static mut global_flags_cache: [u8; 64] = [0; 64];
static mut end_frame: u32 = 0;
pub static mut visible: bool = false;
static mut flag: Flag = Flag(0, 0);

pub fn get_flag_str(f: Flag) -> Option<&'static str> {
    for &(text, known_flag) in FLAGS.iter() {
        if known_flag == f {
            return Some(text);
        }
    }
    None
}

pub fn check_global_flags() {
    if unsafe { visible } && get_frame_count() > unsafe { end_frame } {
        unsafe {
            visible = false;
        }
        let mut console = Console::get();
        console.visible = false;
        console.background_color.a = 150;
        console.lines[1].visible = true;
    }
    if unsafe { visible } {
        let mut console = Console::get(); 
        console.visible = true;
        console.background_color.a = 0;
        console.lines[1].visible = false;
        let Flag(addr, bit) = unsafe { flag };
        if let Some(text) = get_flag_str(Flag(addr, 1 << bit)) {
            let text = if text.len() > 50 {
                &text[..50]
            } else {
                text
            };
            let _ = write!(console.lines[0].begin(), "{}", text);
        } else {
            let _ = write!(console.lines[0].begin(), "Flag {:02X} {} has been set", 0xFF & addr, bit);
        }
    }

    for (index, cached_value) in unsafe { global_flags_cache.iter_mut().enumerate() } {
        let addr = 0x803B872C + index;
        let current_value = read::<u8>(addr);
        let diff = current_value & (0xFF ^ *cached_value);
        if diff != 0 {
            for bit in 0..8 {
                if diff & (1 << bit) != 0 {
                    show_popup(addr, bit);
                }
            }
            *cached_value |= diff;
        }
    }
}

fn show_popup(addr: Addr, bit: u8) {
    unsafe {
        end_frame = get_frame_count() + 200;
        visible = true;
        flag = Flag(addr, bit);
    }
}
