mod utils;

use once_cell::sync::Lazy;
use std::{
    sync::Mutex,
    thread::spawn
};

use utils::utils::game::*;
use utils::utils::utils::*;
use utils::offsets::offsets::offsets;
use device_query::{DeviceQuery, DeviceState, Keycode};

#[allow(non_upper_case_globals)]
static character: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
#[allow(non_upper_case_globals)]
static stop_thread: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

#[allow(dead_code)]
fn is_key_down(key: Keycode) -> bool {
    DeviceState::new().get_keys().contains(&key)
}


fn new_game()   {
    let _thread = spawn(move || {
        let (roblox_prc, real_dm, _base_address) = Game::get_datamodel();
        let utils = Utils::new(&roblox_prc);
        let _game = Game::new(&roblox_prc, &utils, real_dm);

        while !*stop_thread.lock().unwrap() {
            let char = character.lock().unwrap().clone();
            if char == 0 { continue }

            utils.walkspeed(char, 70.0);
        }
    });

}

fn main() {
    let (roblox_prc, old_dm, base_address) = Game::get_datamodel();
    let mut old_placeid = roblox_prc.read_memory::<usize>( old_dm + offsets["PlaceId"] ).unwrap();

    println!("Datamodel: {:#X}\nBase address: {:#X}\nPlaceId: {}", old_dm, base_address, old_placeid);

    new_game();
    loop {
        let (roblox_prc, real_dm, _base_address) = Game::get_datamodel();
        let place_id = roblox_prc.read_memory::<usize>(real_dm + offsets["PlaceId"]).unwrap();

        // if place_id == 0 { continue }

        let utils = Utils::new(&roblox_prc);
        let game = Game::new(&roblox_prc, &utils, real_dm);
        if place_id != old_placeid {
            println!("Place changed: {} -> {}\nDatamodel: {:#X} -> {:#X}", old_placeid, place_id, old_dm, real_dm);
            old_placeid = place_id;

            {
                let mut stop = stop_thread.lock().unwrap();
                *stop = true;
                *stop = false;
            }

            new_game();
        }

        let mut char = character.lock().unwrap();
        *char = game.get_character();

    }
}