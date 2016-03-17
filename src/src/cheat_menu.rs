use libtww::prelude::*;
use libtww::game::{controller, Console};
use libtww::Link;
use libtww::system;

use utils::*;
use cursor;

static mut scroll_offset: usize = 0;

pub fn transition_into() {
    unsafe {
        scroll_offset = 0;
    }
}

pub fn scroll_move_cursor(len: usize) {
    if is_pressed(controller::DPAD_UP) && unsafe { cursor } > 0 {
        unsafe {
            cursor -= 1;
            if cursor >= 4 && cursor - 4 < scroll_offset {
                scroll_offset = cursor - 4;
            }
        }
    } else if is_pressed(controller::DPAD_DOWN) && unsafe { cursor + 1 } < len {
        unsafe {
            cursor += 1;
            if cursor + 4 < len && cursor > scroll_offset + 20 {
                scroll_offset = cursor - 20;
            }
        }
    }
}

struct Cheat {
    id: CheatId,
    name: &'static str,
    active: bool,
    togglable: bool,
}

impl Cheat {
    const fn new(id: CheatId, name: &'static str, togglable: bool) -> Self {
        Cheat {
            id: id,
            name: name,
            active: false,
            togglable: togglable,
        }
    }
}

pub fn apply_cheats() {
    let mut link = Link::get();

    for cheat in unsafe { &cheats } {
        if cheat.active {
            match cheat.id {
                MoonJump => {
                    Link::position().y += 175.0;
                }
                Invincible => {
                    link.heart_quarters = link.heart_pieces;
                }
                InfiniteMagic => {
                    link.max_magic = 32;
                    link.magic = link.max_magic;
                }
                SwiftWind => {
                    let direction = Link::direction();
                    let wind = if direction < 0x1000 || direction > 0xF000 {
                        2
                    } else if direction < 0x3000 {
                        1
                    } else if direction < 0x5000 {
                        0
                    } else if direction < 0x7000 {
                        7
                    } else if direction < 0x9000 {
                        6
                    } else if direction < 0xB000 {
                        5
                    } else if direction < 0xD000 {
                        4
                    } else {
                        3
                    };
                    system::set_wind(wind);
                }
                InfiniteAir => {
                    Link::set_air_meter(900);
                }
                FastMovement => {
                    system::memory::write::<u32>(0x8039830C, 0x40000000);
                    system::memory::write::<u32>(0x80398310, 0x41000000);
                }
                InfiniteBombs => {
                    system::memory::write::<u8>(0x803B8172, 0x63);
                }
                InfiniteArrows => {
                    system::memory::write::<u8>(0x803B8171, 0x63);
                }
                ReloadArea => {
                    system::memory::write::<u8>(0x803BD254, 0x1);
                }
                MapSelect => {
                    system::memory::write::<u32>(0x80AF03E4, 0x8038772C);
                }

            }
        }
        else {
            match cheat.id {
                MapSelect => {
                    // Set the value back to normal if the cheat is not active
                    system::memory::write::<u32>(0x80AF03E4, 0x8038824C);
                }
                _ => {}
            }
        }
    }
}

static mut cheats: [Cheat; 10] = [Cheat::new(Invincible, "Invincible", true),
                                 Cheat::new(InfiniteMagic, "Infinite Magic", true),
                                 Cheat::new(InfiniteAir, "Infinite Air", true),
                                 Cheat::new(SwiftWind, "Swift Wind", true),
                                 Cheat::new(MoonJump, "Moon Jump", false),
                                 Cheat::new(FastMovement, "Fast Movement", false),
                                 Cheat::new(InfiniteBombs, "Infinite Bombs", true),
                                 Cheat::new(InfiniteArrows, "Infinite Arrows", true),
                                 Cheat::new(ReloadArea, "Reload Area", false),
                                 Cheat::new(MapSelect, "Map Select", true)];

#[derive(Copy, Clone)]
enum CheatId {
    Invincible,
    InfiniteMagic,
    InfiniteAir,
    SwiftWind,
    MoonJump,
    FastMovement,
    InfiniteBombs,
    InfiniteArrows,
    ReloadArea,
    MapSelect,
}

use self::CheatId::*;

pub fn render() {
    let console = Console::get();
    let mut lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Cheat Menu");
    let _ = write!(lines[1].begin(), "==========");

    let down_a = is_down(controller::A);
    let pressed_a = is_pressed(controller::A);
    let pressed_b = is_pressed(controller::B);
    // let dpad_left = is_pressed(controller::DPAD_LEFT);
    // let dpad_right = is_pressed(controller::DPAD_RIGHT);

    if pressed_b {
        transition(MenuState::MainMenu);
        return;
    }

    scroll_move_cursor(unsafe { cheats.len() });

    let cheat_id = unsafe { cursor };
    let cheat = unsafe { &mut cheats[cheat_id] };

    if cheat.togglable {
        cheat.active ^= pressed_a;
    } else {
        cheat.active = down_a;
    }

    for (index, (line, cheat)) in lines.into_iter()
                                       .skip(3)
                                       .zip(unsafe {
                                           cheats.iter()
                                                 .skip(scroll_offset)
                                       })
                                       .enumerate()
                                       .take(25) {
        let index = index + unsafe { scroll_offset };
        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }

        let checkbox = if cheat.active {
            "[x] "
        } else {
            "[ ] "
        };

        let text = cheat.name;
        let text = if text.len() > 45 {
            &text[..45]
        } else {
            text
        };

        let _ = write!(line.append(), "{}{}", checkbox, text);
    }
}
