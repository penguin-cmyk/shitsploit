use crate::classes::globals::globals::*;
use crate::utils::cheat::rbx::*;

pub fn entry() {

    /*
        let current_pos = local_player.get_position("HumanoidRootPart");
        println!("Current pos: {:?}", current_pos);
        local_player.set_position("HumanoidRootPart", Vector3::new(0.0, 0.0, 0.0));
    */
    let players_service = GetService("Players".to_string());
    println!("{}" , players_service);

    while *entry_loaded.lock().unwrap() {
        println!("loop while the entry is loaded");
    }
}