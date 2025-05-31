mod utils;

use utils::utils::game::*;
use utils::utils::utils::*;
use utils::offsets::offsets::offsets;
use device_query::{DeviceQuery, DeviceState, Keycode};
use crate::utils::types::Vector3::Vector3;

fn is_key_down(key: Keycode) -> bool {
    DeviceState::new().get_keys().contains(&key)
}



fn main() {
    let (roblox_prc, real_dm, base_address) = Game::get_datamodel();
    let mut old_placeid = roblox_prc.read_memory::<usize>( real_dm + offsets["PlaceId"] ).unwrap();

    println!("Datamodel: {:#X}\nBase address: {:#X}\nPlaceId: {}", real_dm, base_address, old_placeid);

    loop {
        let (roblox_prc, real_dm, base_address) = Game::get_datamodel();
        let mut place_id = roblox_prc.read_memory::<usize>( real_dm + offsets["PlaceId"] ).unwrap();
        if place_id == 0 { continue; }

        if place_id != old_placeid {
            println!("Place changed: {} -> {}", old_placeid, place_id );
            old_placeid = place_id;
        }

        let mut utils = Utils::new(&roblox_prc);


    }


}