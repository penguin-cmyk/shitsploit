mod classes;
mod offsets;
mod utils;

use utils::cheat::rbx;
use crate::classes::globals::globals::*;
use crate::offsets::offsets::*;
fn main() {
    let (real_dm, fake_dm) = rbx::datamodel();
    println!("FDm->RDm: {:#X} -> {:#X}", fake_dm, real_dm);

    let mut old_placeid = process.read_memory::<usize>( real_dm + offsets.PlaceId ).unwrap();

    loop {
        let (real_dm, fake_dm) = rbx::datamodel();
        let place_id = process.read_memory::<usize>( real_dm + offsets.PlaceId ).unwrap();

        if place_id != old_placeid {
            println!("{} -> {}", old_placeid, place_id);
            println!("FDm->RDm: {:#X} -> {:#X}", fake_dm, real_dm);
            old_placeid = place_id;
        }

        if place_id == 0 { continue; }
        let mut game_loaded = loaded.lock().unwrap();
        let player = rbx::GetLocalPlayer();
        {
            if player.character == 0 { continue }
            let health = player.health();
            *game_loaded = (health != -100.0)
        }

        if !(*game_loaded) { continue }
        println!("Health: {}", player.health());
    }
}
