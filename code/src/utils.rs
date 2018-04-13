use libtww::Link;
use libtww::game::Console;
use libtww::link::CollisionType;

use {cheat_menu, controller, flag_menu, inventory_menu, main_menu, quest_menu, spawn_menu,
     warp_menu};

pub fn clear_menu() {
    let console = Console::get();
    let lines = &mut console.lines;
    for line in lines.into_iter().skip(3) {
        line.clear();
    }
}

pub fn transition(state: MenuState) {
    clear_menu();
    unsafe {
        menu_state = state;
    }
    match state {
        MenuState::MainMenu => main_menu::transition_into(),
        MenuState::WarpMenu => warp_menu::transition_into(),
        MenuState::FlagMenu => flag_menu::transition_into(),
        MenuState::InventoryMenu => inventory_menu::transition_into(),
        MenuState::CheatMenu => cheat_menu::transition_into(),
        MenuState::SpawnMenu => spawn_menu::transition_into(),
        MenuState::QuestMenu => quest_menu::transition_into(),
    }
}

pub fn move_cursor(len: usize, cursor: &mut usize) {
    let console = Console::get();
    let lines = &mut console.lines;

    if controller::DPAD_UP.is_pressed() && *cursor > 0 {
        *cursor -= 1;
        while lines[*cursor + 3].len() < 3 {
            *cursor -= 1;
        }
    } else if controller::DPAD_DOWN.is_pressed() && *cursor + 1 < len {
        *cursor += 1;
        while lines[*cursor + 3].len() < 3 {
            *cursor += 1;
        }
    }
}

pub fn next_collision() {
    let collision = Link::collision();
    let collision = match collision {
        CollisionType::Default => CollisionType::ChestStorage,
        CollisionType::ChestStorage => CollisionType::DoorCancel,
        CollisionType::DoorCancel => CollisionType::Default,
    };
    Link::set_collision(collision);
}

pub fn bool_to_text(b: bool) -> &'static str {
    if b {
        "on"
    } else {
        "off"
    }
}

#[derive(Copy, Clone)]
pub enum MenuState {
    MainMenu,
    WarpMenu,
    FlagMenu,
    InventoryMenu,
    CheatMenu,
    SpawnMenu,
    QuestMenu,
}

pub static mut menu_state: MenuState = MenuState::MainMenu;