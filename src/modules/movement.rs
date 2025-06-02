use crate::classes::globals::globals::*;
use crate::offsets;
use crate::process;

#[allow(dead_code)]
pub fn walkspeed(new_speed: f32) {
    let hum = *humanoid.lock().unwrap();
    process.write_memory::<f32>( hum + offsets.WalkSpeed, &new_speed).unwrap();
    process.write_memory::<f32>( hum + offsets.WalkSpeedCheck, &new_speed).unwrap();
}
