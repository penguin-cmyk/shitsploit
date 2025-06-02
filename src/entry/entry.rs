use crate::classes::globals::globals::*;
use crate::classes::rbx::vector3::Vector3;
use crate::modules::movement::walkspeed;
use crate::utils::cheat::rbx::*;

pub fn entry() {
    { let mut entry = entry_loaded.lock().unwrap(); *entry = true }


    let mut local_player = GetLocalPlayer();
    /*
        let current_pos = local_player.get_position("HumanoidRootPart");
        println!("Current pos: {:?}", current_pos);
        local_player.set_position("HumanoidRootPart", Vector3::new(0.0, 0.0, 0.0));
     */
    while *entry_loaded.lock().unwrap() {
        let health = local_player.health();
        println!("Health: {}", health);
        std::thread::sleep(std::time::Duration::from_secs(1));
        local_player.set_health(0.0);
    }
}