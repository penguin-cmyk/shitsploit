use std::collections::HashMap;
use crate::utils::utils::utils::Utils;
use crate::utils::offsets::offsets::offsets;
use crate::utils::types::Vector3::Vector3;

#[allow(non_snake_case)]
pub struct Player<'a> {
    pub Username: String,
    pub Userid: u32,
    pub Character: HashMap<String, usize>,
    pub CharacterAddr: usize,
    pub PlayerAddr: usize,
    pub utils: &'a Utils<'a>,
}

impl<'a> Player<'a> {
    pub fn default(utils: &'a Utils<'a>,) -> Self {
        Player {
            Userid: 0,
            Username: "".to_string(),
            utils,
            CharacterAddr: 0,
            PlayerAddr: 0,
            Character: HashMap::new()
        }
    }
    pub fn health(&self) -> f32 {
        let humanoid = self.Character.get("Humanoid").unwrap();
        let health = self.utils.read_memory::<f32>(*humanoid + offsets["Health"]).unwrap();

        health
    }

    pub fn player_find(&self, option: &str) -> usize {
        self.utils.find_first_child(self.PlayerAddr, option.to_string())
    }

    pub fn get_team(&self) -> usize {
        self.utils.read_memory::<usize>( self.PlayerAddr + offsets["Team"] ).unwrap()
    }

    pub fn can_collide(&self, collide: bool) {
        let mut collision_targets: Vec<&str> = Vec::new();
        if self.Character.contains_key("UpperTorso") {
            collision_targets.push("Head");
            collision_targets.push("UpperTorso");
            collision_targets.push("LowerTorso");
            collision_targets.push("HumanoidRootPart");
        } else {
            collision_targets.push("Head");
            collision_targets.push("Torso");
            collision_targets.push("HumanoidRootPart");
        }

        for part in self.Character.iter() {
            let (name, part) = part;
            if collision_targets.contains(&name.as_str()) {
                let primitive = self.utils.read_memory::<usize>(part + offsets["Primitive"]).unwrap();
                self.utils.write_memory::<bool>(primitive + offsets["CanCollide"], &collide).unwrap()
            }
        }
    }

    pub fn character_find(&self, option: &str) -> usize {
        self.utils.find_first_child(self.CharacterAddr, option.to_string())
    }

    pub fn get_position(&self, part: &str) -> Vector3 {
        let part = self.Character.get(part).unwrap_or(&0);
        if *part == 0 { return Vector3::zero() }

        let primitive = self.utils.read_memory::<usize>(*part + offsets["Primitive"]).unwrap();
        if primitive == 0 { return Vector3::zero() }

        self.utils.read_memory::<Vector3>(primitive + offsets["Position"]).unwrap()
    }

    pub fn set_position(&mut self, part: &str, position: Vector3) {
        let part = self.Character.get(part).unwrap_or(&0);
        if *part == 0 { return }

        let primitive = self.utils.read_memory::<usize>(*part + offsets["Primitive"]).unwrap();
        if primitive == 0 { return; }

        for _ in  0..4000 {
            self.utils.write_memory(primitive + offsets["Position"], &position).unwrap();
        }
    }

    pub fn get_velocity(&self, part: &str) -> Vector3 {
        let part = self.Character.get(part).unwrap_or(&0);
        if *part == 0 { return Vector3::zero() }

        let primitive = self.utils.read_memory::<usize>(*part + offsets["Primitive"]).unwrap();
        if primitive == 0 { return Vector3::zero() }

        self.utils.read_memory::<Vector3>(primitive + offsets["Velocity"]).unwrap()
    }

    pub fn set_velocity(&mut self, part: &str, velocity: Vector3) {
        let part = self.Character.get(part).unwrap_or(&0);
        if *part == 0 { return }

        let primitive = self.utils.read_memory::<usize>(*part + offsets["Primitive"]).unwrap();
        if primitive == 0 { return; }

        for _ in  0..4000 {
            self.utils.write_memory(primitive + offsets["Velocity"], &velocity).unwrap();

        }
    }

    pub fn move_direction(&self) -> Vector3 {
        let humanoid = self.Character.get("Humanoid").unwrap_or(&0);
        if *humanoid == 0 { return Vector3::zero() }

        self.utils.read_memory::<Vector3>(humanoid + offsets["MoveDirection"]).unwrap()
    }

    pub fn change_walkspeed(&self, new_walk_speed: f32) {
        let humanoid = self.Character.get("Humanoid").unwrap_or(&0);
        if *humanoid == 0 { return }

        self.utils.write_memory::<f32>(*humanoid + offsets["WalkSpeed"], &new_walk_speed).unwrap();
        self.utils.write_memory::<f32>(*humanoid + offsets["WalkSpeedCheck"], &new_walk_speed).unwrap();
    }

    pub fn change_jumppower(&self, new_walk_speed: f32) {
        let humanoid = self.Character.get("Humanoid").unwrap_or(&0);
        if *humanoid == 0 { return }
        self.utils.write_memory::<f32>(*humanoid + offsets["JumpPower"], &new_walk_speed).unwrap();
    }

}
