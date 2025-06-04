use crate::classes::globals::globals::*;
use crate::classes::rbx::vector3::*;
use crate::offsets::offsets::*;
use crate::utils::cheat::rbx;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub userid: usize,
    pub character: usize,
    pub player: usize,
}

#[allow(dead_code)]
impl Player {
    pub fn primitive(&self, address: usize) -> usize { process.read_memory::<usize>(address + offsets.Primitive).unwrap() }

    pub fn default() -> Player { Player { name: String::from(""), userid: 0, character: 0, player: 0 } }
    pub fn health(&self) -> f32 {
        if self.character == 0 { return -100.0 }

        let hum = humanoid.lock().unwrap();
        if *hum == 0 { return -100.0 }

        let health = process.read_memory::<f32>(*hum + offsets.Health).unwrap();
        drop(hum);

        health
    }

    pub fn set_health(&self, health: f32) {
        if self.character == 0 { return  }

        let hum = humanoid.lock().unwrap();
        if *hum == 0 { return  }

        process.write_memory::<f32>(*hum + offsets.Health, &health).unwrap();
        drop(hum);
    }

    pub fn move_direction(&self) -> Vector3 {
        if self.character == 0 { return Vector3::zero() }

        let hum = humanoid.lock().unwrap();
        if *hum == 0 { return Vector3::zero() }

        rbx::MoveDirection(*hum)
    }

    pub fn team(&self) -> usize {
        process.read_memory::<usize>(self.player + offsets.Team).unwrap()
    }

    pub fn get_position(&self, part: &str) -> Vector3 {
        if self.character == 0 { return Vector3::zero() }
        let part = rbx::FindFirstChild(self.character, part.to_string());
        if part == 0 { return Vector3::zero() }

        let primitive = self.primitive(part);
        process.read_memory::<Vector3>(primitive + offsets.Position).unwrap()
    }

    pub fn set_position(&self, part: &str, position: Vector3) {
        if self.character == 0 { return }
        let part = rbx::FindFirstChild(self.character, part.to_string());
        if part == 0 { return }

        let primitive = self.primitive(part);
        process.write_memory::<Vector3>(primitive + offsets.Position, &position).unwrap()
    }

    pub fn get_velocity(&self, part: &str) -> Vector3 {
        if self.character == 0 { return Vector3::zero() }
        let part = rbx::FindFirstChild(self.character, part.to_string());
        if part == 0 { return Vector3::zero() }

        let primitive = self.primitive(part);
        process.read_memory::<Vector3>(primitive + offsets.Velocity).unwrap()
    }

    pub fn set_velocity(&self, part: &str, position: Vector3) {
        if self.character == 0 { return }
        let part = rbx::FindFirstChild(self.character, part.to_string());
        if part == 0 { return }

        let primitive = self.primitive(part);
        process.write_memory::<Vector3>(primitive + offsets.Velocity, &position).unwrap()
    }

}