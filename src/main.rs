mod utils;

use utils::utils::game::*;
use utils::utils::utils::*;
use utils::offsets::offsets::offsets;
use device_query::{DeviceQuery, DeviceState, Keycode};

fn is_key_down(key: Keycode) -> bool {
    DeviceState::new().get_keys().contains(&key)
}



fn main() {
    let (roblox_prc, real_dm, base_address) = Game::get_datamodel();

    let utils = Utils::new(&roblox_prc);
    let game = Game::new(&roblox_prc, &utils, real_dm);

    let mut localplayer = game.get_localplayer();
    let health = localplayer.health();

    let parts = localplayer.Character.clone();

    println!("Health: {}", health);
    {
        loop {
            localplayer.change_walkspeed(50.0);
            /*
            let mut position = localplayer.get_position("HumanoidRootPart");
            let mut move_direction = localplayer.move_direction();

            localplayer.can_collide(false);
            let team = localplayer.get_team();
            println!("Team: {}", team);
            position.y =  if is_key_down(Keycode::Space) { position.y + 5.0 } else if is_key_down(Keycode::LControl) { position.y -5.0 } else { position.y };
            localplayer.set_position("HumanoidRootPart", (position + move_direction * 10.0));

            if is_key_down(Keycode::X) { break; }
             */
        }
    }

}