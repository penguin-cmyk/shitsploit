mod utils;

use utils::utils::game::*;
use utils::utils::utils::*;
use utils::offsets::offsets::offsets;
use device_query::{DeviceQuery, DeviceState, Keycode};
fn is_key_down(key: Keycode) -> bool {
    DeviceState::new().get_keys().contains(&key)
}

fn main() {
    let (roblox_prc, old_dm, base_address) = Game::get_datamodel();
    let mut old_placeid = roblox_prc.read_memory::<usize>( old_dm + offsets["PlaceId"] ).unwrap();

    println!("Datamodel: {:#X}\nBase address: {:#X}\nPlaceId: {}", old_dm, base_address, old_placeid);

    let mut character_addr = 0;
    loop {
        let (roblox_prc, real_dm, base_address) = Game::get_datamodel();
        let mut place_id = roblox_prc.read_memory::<usize>( real_dm + offsets["PlaceId"] ).unwrap();
        if place_id == 0 { continue; }

        if place_id != old_placeid {
            println!("Place changed: {} -> {}\nDatamodel: {:#X} -> {:#X}\nCharacter: {:#X}", old_placeid, place_id, old_dm, real_dm, character_addr );
            old_placeid = place_id;
        }

        let utils = Utils::new(&roblox_prc);
        let mut game = Game::new(&roblox_prc, &utils, real_dm);
        let mut localplayer = game.get_localplayer();

        if localplayer.CharacterAddr == 0 { continue; }
        character_addr = localplayer.CharacterAddr;

        /*
            let mut position = localplayer.get_position("HumanoidRootPart");
            let mut move_direction = localplayer.move_direction();

            localplayer.can_collide(false);
            position.y =  if is_key_down(Keycode::Space) { position.y + 5.0 } else if is_key_down(Keycode::LControl) { position.y -5.0 } else { position.y };

            localplayer.set_velocity("HumanoidRootPart", Vector3::zero());
            localplayer.set_position("HumanoidRootPart", (position + move_direction * 10.0));
            if is_key_down(Keycode::X) { break; }
         */
    }


}