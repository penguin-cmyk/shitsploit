mod classes;
mod offsets;
mod utils;
mod modules;
mod entry;

use utils::cheat::rbx;
use entry::entry::entry;
use classes::globals::globals::*;
use offsets::offsets::*;
fn main() {

    let (real_dm, fake_dm) = rbx::datamodel();
    println!("FDm->RDm: {:#X} -> {:#X}", fake_dm, real_dm);

    let mut old_placeid = process.read_memory::<usize>( real_dm + offsets.PlaceId ).unwrap();

    loop {
        let (real_dm, _) = rbx::datamodel();
        let place_id = process.read_memory::<usize>( real_dm + offsets.PlaceId ).unwrap();

        if place_id != old_placeid { println!("{} -> {}", old_placeid, place_id); old_placeid = place_id; }

        if place_id == 0 {
            let mut entry_l = entry_loaded.lock().unwrap();
            *entry_l = false;
            continue
        }
        let mut game_loaded = loaded.lock().unwrap();
        let player = rbx::GetLocalPlayer();

        {
            if player.character == 0 { continue }
            let health = player.health();
            *game_loaded = health != -100.0; // if something is 0 it will return -100.0 so we can check for that

            // have these both as globals so that we don't need to access them everytime
            let mut char = character.lock().unwrap();
            *char = player.character;

            let mut hum = humanoid.lock().unwrap();
            *hum = rbx::FindFirstChild(*char, "Humanoid".to_string());

            if *hum == 0 { continue }
        }

        if !(*game_loaded) {
            let mut entry_l = entry_loaded.lock().unwrap();
            *entry_l = false; // so we can check inside entry and also stop it and reload it
            continue
        }

        {
            let mut entry_l = entry_loaded.lock().unwrap();
            if *entry_l { continue }
            *entry_l = true
        }

        std::thread::spawn(move || { entry() });
    }
}
