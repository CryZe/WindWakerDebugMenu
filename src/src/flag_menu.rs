use libtww::prelude::*;
use libtww::game::{controller, Console};
use libtww::game::flag::*;

use utils::*;
use cursor;

pub const FLAGS: [(&'static str, Flag); 167] =
    [("Has seen Helmaroc arriving at Outset", HAS_SEEN_HELMAROC_ARRIVING_AT_OUTSET),
     ("Forest of Fairies Bokoblins Spawned", FOREST_OF_FAIRIES_BOKOBLINS_SPAWNED),
     ("Talked to Abe on Outset after Helmaroc", TALKED_TO_ABE_ON_OUTSET_AFTER_HELMAROC),
     ("Talked to Mesa on Outset with Sword", TALKED_TO_MESA_ON_OUTSET_WITH_SWORD),
     ("Talked to Mesa on Outset without Sword", TALKED_TO_MESA_ON_OUTSET_WITHOUT_SWORD),
     ("Talked to Abe on Outset before Helmaroc", TALKED_TO_ABE_ON_OUTSET_BEFORE_HELMAROC),
     ("Rescued Tetra", RESCUED_TETRA),
     ("Got a Rupee on Outset Rocks", GOT_A_RUPEE_ON_THE_OUTSET_ROCKS),
     ("Talked to Orca after Helmaroc arrived", TALKED_TO_ORCA_AFTER_HELMAROC_ARRIVED),
     ("Talked to Sue-Belle with Pirates on Outset", TALKED_TO_SUE_BELLE_WITH_PIRATES_ON_OUTSET),
     ("Talked to Sue-Belle on Outset after Helmaroc", TALKED_TO_SUE_BELLE_ON_OUTSET_AFTER_HELMAROC),
     ("Talked to Sue-Belle on Outset before Helmaroc", TALKED_TO_SUE_BELLE_ON_OUTSET_BEFORE_HELMAROC),
     ("Saw a Light Operator Bokoblin", SAW_A_LIGHT_OPERATOR_BOKOBLIN),
     ("Talked to Joel on Outset", TALKED_TO_JOEL_ON_OUTSET),
     ("Talked to Zill on Outset East", TALKED_TO_ZILL_ON_OUTSET_EAST),
     ("Talked to Kids with Pirates on Outset", TALKED_TO_KIDS_WITH_PIRATES_ON_OUTSET),
     ("Saw Tetra in Forest of Fairies", SAW_TETRA_IN_FOREST_OF_FAIRIES),
     ("Killed one Forest of Fairies Bokoblin", KILLED_ONE_FOREST_OF_FAIRIES_BOKOBLIN),
     ("Brought back 1 Pig to Rose on Outset", BROUGHT_BACK_1_PIG_TO_ROSE_ON_OUTSET),
     ("Talked to Rose on Outset 2", TALKED_TO_ROSE_ON_OUTSET_2),
     ("Talked to Rose with Pirates on Outset", TALKED_TO_ROSE_WITH_PIRATES_ON_OUTSET),
     ("Pirate Ship Arriving On Outset", PIRATE_SHIP_ARRIVING_ON_OUTSET),
     ("Saw Lava Platforms forming", SAW_LAVA_PLATFORMS_FORMING),
     ("Picked up first Barrel in FF1", PICKED_UP_FIRST_BARREL_IN_FF1),
     ("Grabbed first Rope in FF1", GRABBED_FIRST_ROPE_IN_FF1),
     ("Got thrown into Jail in FF1", GOT_THROWN_INTO_JAIL_IN_FF1),
     ("Grappled Valoo's Tail", GRAPPLED_VALOOS_TAIL),
     ("Killed both Forest of Fairies Bokoblins", KILLED_BOTH_FOREST_OF_FAIRIES_BOKOBLINS),
     ("Bonked Orca's Wall", BONKED_ORCAS_WALL),
     ("Visited Sturgeon on Outset (Begin)", VISITED_STURGEON_ON_OUTSET_BEGIN),
     ("Visited Sturgeon on Outset (End)", VISITED_STURGEON_ON_OUTSET_END),
     ("Talked to Sturgeon on Outset before Helmaroc", TALKED_TO_STURGEON_ON_OUTSET_BEFORE_HELMAROC),
     ("Gossip Stone at FF1", GOSSIP_STONE_AT_FF1),
     ("Grappled Valoo's Tail 2", GRAPPLED_VALOOS_TAIL_2),
     ("Used Grappling Hook", USED_GRAPPLING_HOOK),
     ("Talked to Grandma after seeing Helmaroc", TALKED_TO_GRANDMA_AFTER_SEEING_HELMAROC),
     ("Talked to Grandma after getting Sword", TALKED_TO_GRANDMA_AFTER_GETTING_SWORD),
     ("Visited Orca before Helmaroc", VISITED_ORCA_BEFORE_HELMAROC),
     ("Talked to Mako on sailing Pirate Ship", TALKED_TO_MAKO_ON_SAILING_PIRATE_SHIP),
     ("Talked to Zuko on sailing Pirate Ship", TALKED_TO_ZUKO_ON_SAILING_PIRATE_SHIP),
     ("Talked to Niko after Minigame", TALKED_TO_NIKO_AFTER_MINIGAME),
     ("Completed Pirate Ship Minigame", COMPLETED_PIRATE_SHIP_MINIGAME),
     ("Saw Pirate Ship Minigame Intro", SAW_PIRATE_SHIP_MINIGAME_INTRO),
     ("Talked to Grandma after Aryll got captured", TALKED_TO_GRANDMA_AFTER_ARYLL_GOT_CAPTURED),
     ("Got catapulted to FF1 / Spawn there", GOT_CATAPULTED_TO_FF1_AND_SPAWN_THERE),
     ("Long Tetra Text on Outset", LONG_TETRA_TEXT_ON_OUTSET),
     ("Tetra told you to climb up the ladder", TETRA_TOLD_YOU_TO_CLIMB_UP_THE_LADDER),
     ("Completed Pirate Ship Minigame / Spawn on Pirate Ship", COMPLETED_PIRATE_SHIP_MINIGAME_AND_SPAWN_ON_PIRATE_SHIP),
     ("Talked to Tetra on sailing Pirate Ship", TALKED_TO_TETRA_ON_SAILING_PIRATE_SHIP),
     ("Talked to Nudge on sailing Pirate Ship", TALKED_TO_NUDGE_ON_SAILING_PIRATE_SHIP),
     ("Talked to Senza on sailing Pirate Ship", TALKED_TO_SENZA_ON_SAILING_PIRATE_SHIP),
     ("Talked to Gonzo on sailing Pirate Ship", TALKED_TO_GONZO_ON_SAILING_PIRATE_SHIP),
     ("Triggered Map Fish", TRIGGERED_MAP_FISH),
     ("Saw Dragon Roost Island Intro", SAW_DRAGON_ROOST_ISLAND_INTRO),
     ("Sail Introduction Text / Map unlocked", SAIL_INTRODUCTION_TEXT_AND_MAP_UNLOCKED),
     ("Talked to Gonzo on Outset Beach", TALKED_TO_GONZO_ON_OUTSET_BEACH),
     ("Talked to Niko on Outset Beach", TALKED_TO_NIKO_ON_OUTSET_BEACH),
     ("Endless Night", ENDLESS_NIGHT),
     ("Talked to KoRL on DRI", TALKED_TO_KORL_ON_DRI),
     ("Talked to Pompie and Vera on Windfall", TALKED_TO_POMPIE_AND_VERA_ON_WINDFALL),
     ("KoRL Din's Pearl Text allowing you to enter him", KORL_DINS_PEARL_TEXT_ALLOWING_YOU_TO_ENTER_HIM),
     ("Talked to Abe with Pirates on Outset", TALKED_TO_ABE_WITH_PIRATES_ON_OUTSET),
     ("Talked positively to Mila's Father on Windfall", TALKED_POSITIVELY_TO_MILAS_FATHER_ON_WINDFALL),
     ("Talked positively to Maggie's Father on Windfall", TALKED_POSITIVELY_TO_MAGGIES_FATHER_ON_WINDFALL),
     ("Talked to Tott on Windfall", TALKED_TO_TOTT_ON_WINDFALL),
     ("Hurricane Spin unlocked", HURRICANE_SPIN_UNLOCKED),
     ("Talked to Tingle in Jail", TALKED_TO_TINGLE_IN_JAIL),
     ("Rescued Tingle", RESCUED_TINGLE),
     ("Got Komali's Letter", GOT_KOMALIS_LETTER),
     ("Played Sploosh Kaboom well", PLAYED_SPLOOSH_KABOOM_WELL),
     ("Pirates on Outset", PIRATES_ON_OUTSET),
     ("Talked to Komali after showing the Letter", TALKED_TO_KOMALI_AFTER_SHOWING_THE_LETTER),
     ("Talked to Komali without the Letter", TALKED_TO_KOMALI_WITHOUT_THE_LETTER),
     ("KoRL unlocked / Spawn on Windfall", KORL_UNLOCKED_AND_SPAWN_ON_WINDFALL),
     ("Watched Fire and Ice Arrows Cutscene", WATCHED_FIRE_AND_ICE_ARROWS_CUTSCENE),
     ("Got Grappling Hook from Medli", GOT_GRAPPLING_HOOK_FROM_MEDLI),
     ("Got Bottle from Medli", GOT_BOTTLE_FROM_MEDLI),
     ("Talked to Medli in the Pond", TALKED_TO_MEDLI_IN_THE_POND),
     ("Talked to Doc Bandam on Windfall", TALKED_TO_DOC_BANDAM_ON_WINDFALL),
     ("Rescued Medli in DRC", RESCUED_MEDLI_IN_DRC),
     ("Talked to Jun-Roberto on Windfall", TALKED_TO_JUN_ROBERTO_ON_WINDFALL),
     ("Talked to Jin on Windfall", TALKED_TO_JIN_ON_WINDFALL),
     ("Talked to Jan on Windfall", TALKED_TO_JAN_ON_WINDFALL),
     ("Talked to Lenzo on Windfall without Camera", TALKED_TO_LENZO_ON_WINDFALL_WITHOUT_CAMERA),
     ("Talked to Ivan on Windfall", TALKED_TO_IVAN_ON_WINDFALL),
     ("Medli explained Grappling Hook", MEDLI_EXPLAINED_GRAPPLING_HOOK),
     ("Agreed to help Mrs. Marie", AGREED_TO_HELP_MRS_MARIE),
     ("Talked to Quill on Outset Beach", TALKED_TO_QUILL_ON_OUTSET_BEACH),
     ("Watched Medli on DRI Introduction Cutscene", WATCHED_MEDLI_ON_DRI_INTRODUCTION_CUTSCENE),
     ("Showed Medli the Wind Waker On DRI", SHOWED_MEDLI_THE_WIND_WAKER_ON_DRI),
     ("Talked to all the Kids on Windfall", TALKED_TO_ALL_THE_KIDS_ON_WINDFALL),
     ("Has Makar on Boat 1", HAS_MAKAR_ON_BOAT_1),
     ("Has Medli on Boat 1", HAS_MEDLI_ON_BOAT_1),
     ("Has Makar on Boat 2 / Is grabbable", HAS_MAKAR_ON_BOAT_2_AND_IS_GRABBABLE),
     ("Has Medli on Boat 2 / Can carry you", HAS_MEDLI_ON_BOAT_2_AND_CAN_CARRY_YOU),
     ("Talked to Lenzo on Windfall with Camera", TALKED_TO_LENZO_ON_WINDFALL_WITH_CAMERA),
     ("Saw Komali in his Room", SAW_KOMALI_IN_HIS_ROOM),
     ("Talked to KoRL after Hyrule 2 / FF3 active", TALKED_TO_KORL_AFTER_HYRULE_2_AND_FF3_ACTIVE),
     ("Is allowed to enter KoRL", IS_ALLOWED_TO_ENTER_KORL),
     ("Left Dragon Roost Island Quadrant", LEFT_DRAGON_ROOST_ISLAND_QUADRANT),
     ("Talked to Gossack on Windfall", TALKED_TO_GOSSACK_ON_WINDFALL),
     ("Talked to Minenco on Windfall", TALKED_TO_MINENCO_ON_WINDFALL),
     ("Talked to Missy on Windfall", TALKED_TO_MISSY_ON_WINDFALL),
     ("Talked to Mrs. Marie on Windfall", TALKED_TO_MRS_MARIE_ON_WINDFALL),
     ("Talked to Garrickson on Windfall", TALKED_TO_GARRICKSON_ON_WINDFALL),
     ("Talked to Gillian on Windfall", TALKED_TO_GILLIAN_ON_WINDFALL),
     ("Tower Of The Gods raised", TOWER_OF_THE_GODS_RAISED),
     ("Saw Quill Cutscene on DRI", SAW_QUILL_CUTSCENE_ON_DRI),
     ("Talked to Linda on Windfall", TALKED_TO_LINDA_ON_WINDFALL),
     ("Talked to Anton on Windfall", TALKED_TO_ANTON_ON_WINDFALL),
     ("Talked to Gummy on Windfall", TALKED_TO_GUMMY_ON_WINDFALL),
     ("Watched Departure Cutscene / Spawn on Pirate Ship", WATCHED_DEPARTURE_CUTSCENE_AND_SPAWN_ON_PIRATE_SHIP),
     ("Talked To Zunari on Windfall", TALKED_TO_ZUNARI_ON_WINDFALL),
     ("Talked to Dampa on Windfall", TALKED_TO_DAMPA_ON_WINDFALL),
     ("Talked to Sam on Windfall", TALKED_TO_SAM_ON_WINDFALL),
     ("Played Sploosh Kaboom", PLAYED_SPLOOSH_KABOOM),
     ("Got Delivery Bag", GOT_DELIVERY_BAG),
     ("Watched Find Sister in FF1 Cutscene", WATCHED_FIND_SISTER_IN_FF1_CUTSCENE),
     ("Learned Wind's Requiem", LEARNED_WINDS_REQUIEM),
     ("Some weird Fire and Ice Arrows Cutscene Flag", SOME_WEIRD_FIRE_AND_ICE_ARROWS_CUTSCENE_FLAG),
     ("Made Mila's Father angry", MADE_MILAS_FATHER_ANGRY),
     ("Talked to Joanna on Windfall", TALKED_TO_JOANNA_ON_WINDFALL),
     ("Denied leaving Outset", DENIED_LEAVING_OUTSET),
     ("Makar in Wind Temple", MAKAR_IN_WIND_TEMPLE),
     ("Medli in Earth Temple", MEDLI_IN_EARTH_TEMPLE),
     ("Enter KoRL for the first time / Spawn anywhere", ENTER_KORL_FOR_THE_FIRST_TIME_AND_SPAWN_ANYWHERE),
     ("Has Hero's Clothes", HAS_HEROS_CLOTHES),
     ("Talked to Potova on Windfall", TALKED_TO_POTOVA_ON_WINDFALL),
     ("Mighty Darknuts defeated", MIGHTY_DARKNUTS_DEFEATED),
     ("Barrier down", BARRIER_DOWN),
     ("Talked to Rose on Outset 1", TALKED_TO_ROSE_ON_OUTSET_1),
     ("Animation Set 2", ANIMATION_SET_2),
     ("Tetra to Zelda Cutscene", TETRA_TO_ZELDA_CUTSCENE),
     ("Master Sword Cutscene", MASTER_SWORD_CUTSCENE),
     ("Hyrule 3 Warp Cutscene", HYRULE_3_WARP_CUTSCENE),
     ("Ringing Bell and Hyrule 1 Cutscene", RINGING_BELL_AND_HYRULE_1_CUTSCENE),
     ("Wind God's Aria Cutscene", WIND_GODS_ARIA_CUTSCENE),
     ("Earth God's Lyric Cutscene", EARTH_GODS_LYRIC_CUTSCENE),
     ("Watched meeting KoRL Cutscene", WATCHED_MEETING_KORL_CUTSCENE),
     ("Makar in Wind Temple Entrance", MAKAR_IN_WIND_TEMPLE_ENTRANCE),
     ("Medli in Earth Temple Entrance", MEDLI_IN_EARTH_TEMPLE_ENTRANCE),
     ("Pearl Tower Cutscene", PEARL_TOWER_CUTSCENE),
     ("Did Sword Fighting Tutorial", DID_SWORD_FIGHTING_TUTORIAL),
     ("Did early Sword Fighting Tutorial", DID_EARLY_SWORD_FIGHTING_TUTORIAL),
     ("Talked to Zill on Outset West", TALKED_TO_ZILL_ON_OUTSET_WEST),
     ("Saw Shield is missing", SAW_SHIELD_IS_MISSING),
     ("Watched Descending Down To Hyrule 2 Cutscene", WATCHED_DESCENDING_DOWN_TO_HYRULE_2_CUTSCENE),
     ("Watched Text after Fire & Ice Arrows Cutscene", WATCHED_TEXT_AFTER_FIRE_AND_ICE_ARROWS_CUTSCENE),
     ("Has seen Intro", HAS_SEEN_INTRO),
     ("Mighty Darknuts spawned", MIGHTY_DARKNUTS_SPAWNED),
     ("Talked to Sturgeon on Outset after Helmaroc", TALKED_TO_STURGEON_ON_OUTSET_AFTER_HELMAROC),
     ("Colors in Hyrule", COLORS_IN_HYRULE),
     ("Watched Courtyard Cutscene", WATCHED_COURTYARD_CUTSCENE),
     ("Moved Statue in Hyrule", MOVED_STATUE_IN_HYRULE),
     ("Watched FF2 Ganondorf Cutscene", WATCHED_FF2_GANONDORF_CUTSCENE),
     ("Hyrule 3 Electrical Barrier Cutscene 1", HYRULE_3_ELECTRICAL_BARRIER_CUTSCENE_1),
     ("Trials Jalhalla", TRIALS_JALHALLA),
     ("Trials Kalle Demos", TRIALS_KALLE_DEMOS),
     ("Trials Gohma", TRIALS_GOHMA),
     ("Saw DRC Beaten Cutscene", SAW_DRC_BEATEN_CUTSCENE),
     ("Trials Molgera", TRIALS_MOLGERA),
     ("Pulled Master Sword In Hyrule 1 Swinging Cutscene", PULLED_MASTER_SWORD_IN_HYRULE_1_SWINGING_CUTSCENE),
     ("Watched Medli Dragon Roost Cutscene", WATCHED_MEDLI_DRAGON_ROOST_CUTSCENE),
     ("Hyrule 1 Electrical Barrier deactivated", HYRULE_1_ELECTRICAL_BARRIER_DEACTIVATED),
     ("FF3 to Hyrule Warp active", FF3_TO_HYRULE_WARP_ACTIVE),
     ("Defeated Gohma", DEFEATED_GOHMA),
     ("Dont show Weapons", DONT_SHOW_WEAPONS)];

static mut scroll_offset: usize = 0;

pub fn transition_into() {
    unsafe {
        scroll_offset = 0;
    }
}

pub fn scroll_move_cursor() {
    if is_pressed(controller::DPAD_UP) && unsafe { cursor } > 0 {
        unsafe {
            cursor -= 1;
            if cursor >= 4 && cursor - 4 < scroll_offset {
                scroll_offset = cursor - 4;
            }
        }
    } else if is_pressed(controller::DPAD_DOWN) && unsafe { cursor + 1 } < FLAGS.len() {
        unsafe {
            cursor += 1;
            if cursor + 4 < FLAGS.len() && cursor > scroll_offset + 20 {
                scroll_offset = cursor - 20;
            }
        }
    }
}

pub fn render() {
    let console = Console::get();
    let mut lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Flag Menu");
    let _ = write!(lines[1].begin(), "=========");

    let pressed_a = is_pressed(controller::A);
    let pressed_b = is_pressed(controller::B);

    if pressed_b {
        transition(MenuState::MainMenu);
        return;
    }

    scroll_move_cursor();

    if pressed_a {
        let (_, flag) = FLAGS[unsafe { cursor }];
        flag.toggle();
    }

    for (index, (line, &(text, flag))) in lines.into_iter()
                                               .skip(3)
                                               .zip(FLAGS.iter().skip(unsafe { scroll_offset }))
                                               .enumerate()
                                               .take(25) {
        if index == unsafe { cursor - scroll_offset } {
            let _ = write!(line.begin(), "> [");
        } else {
            let _ = write!(line.begin(), "  [");
        }
        let text = if text.len() > 45 {
            &text[..45]
        } else {
            text
        };
        let active = if flag.is_active() {
            'x'
        } else {
            ' '
        };
        let _ = write!(line.append(), "{}] {}", active, text);
    }
}
