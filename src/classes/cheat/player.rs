use crate::classes::globals::globals::*;
use crate::classes::rbx::vector3::*;
use crate::offsets::offsets::*;
use crate::utils::cheat::rbx;

pub struct Player {
    pub name: String,
    pub userid: usize,
    pub character: usize,
    pub player: usize,
}


impl Player {
    fn primitive(&self, address: usize) -> usize { process.read_memory::<usize>(address + offsets.Primitive).unwrap() }

    pub fn default() -> Player { Player { name: String::from(""), userid: 0, character: 0, player: 0 } }
    pub fn health(&self) -> f32 {
        if self.character == 0 { return 0.0 }

        let humanoid = rbx::FindFirstChild(self.character, "Humanoid".to_string());
        if humanoid == 0 { return -100.0 }

        let health = process.read_memory::<f32>(humanoid + offsets.Health).unwrap();
        health
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
}