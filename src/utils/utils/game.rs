use std::collections::HashMap;
use memory_utils::process::Process;


use crate::utils::utils::utils::Utils;
use crate::utils::types::Vector3::*;
use crate::utils::types::Player::*;
use crate::utils::offsets::offsets::offsets;


pub struct Game<'a> {
    pub process: &'a Process,
    pub utils: &'a Utils<'a>,
    pub datamodel: usize,
}


impl<'a> Game<'a> {
    pub fn get_datamodel() -> (Process, usize , usize) {
        let roblox_prc = Process::new(Process::pid("RobloxPlayerBeta.exe").unwrap());
        let base_address = roblox_prc.get_base_address().unwrap() as usize;

        let rebase = |address: usize| -> usize { base_address + address };
        let fake_dm_pointer = rebase(0x66EA5E8);

        let fake_dm = roblox_prc.read_memory::<usize>(fake_dm_pointer).unwrap();
        if fake_dm == 0 {
            println!("Failed to get the fake datamodel");
            return (roblox_prc, 0, base_address)
        }

        let real_dm = roblox_prc.read_memory::<usize>(fake_dm + 0x1B8).unwrap();

        (roblox_prc, real_dm, base_address)
    }

    pub fn new(process: &'a Process, utils: &'a Utils, datamodel: usize) -> Self {
        Self {
            process,
            utils,
            datamodel ,

        }
    }

    pub fn get_service(&self, name: &str) -> usize {
         self.utils.find_first_child(self.datamodel, name.to_string())
    }

    pub fn view(&self, camera: usize, humanoid: usize) {
        self.process.write_memory::<usize>(camera + offsets["CameraSubject"], &humanoid).unwrap();
    }

    pub fn camera_pos(&self, camera: usize) -> Vector3 {
        self.process.read_memory::<Vector3>(camera + offsets["CameraPosition"]).unwrap()
    }

    pub fn get_players(&self) -> Vec<Player> {
        let mut players = Vec::new();
        let players_addr = self.utils.find_first_child(self.datamodel, "Players".to_string());
        let player_children = self.utils.getchildren(players_addr);

        for player in player_children {
            let player_name = self.utils.get_name(player).unwrap_or_default();
            let mut character_hash_map = HashMap::<String, usize>::new();

            let character = self.process.read_memory::<usize>(player + offsets["ModelInstance"]).unwrap();
            let character_children = self.utils.getchildren(character);

            for character_child in character_children {
                let part_name = self.utils.get_name(character_child).unwrap_or_default();
                character_hash_map.insert(part_name, character_child);
            }

            players.push(Player {
                Username: player_name,
                Userid: self.process.read_memory::<u32>(player + offsets["UserId"]).unwrap_or_default(),
                Character: character_hash_map,
                CharacterAddr: character,
                PlayerAddr: player,
                utils: self.utils,

            });
        }

        players
    }


    pub fn get_localplayer(&self) -> Player {
        let players_addr = self.utils.find_first_child(self.datamodel, "Players".to_string());
        let localplayer = self.process.read_memory::<usize>(players_addr + offsets["LocalPlayer"]).unwrap();

        let player_name = self.utils.get_name(localplayer).unwrap_or_default();
        let player_userid = self.process.read_memory::<u32>(localplayer + offsets["UserId"]).unwrap_or_default();

        let mut character = self.process.read_memory::<usize>(localplayer + offsets["ModelInstance"] ).unwrap();
        while character == 0 {
            character = self.process.read_memory::<usize>(localplayer + offsets["ModelInstance"]).unwrap();
        }


        let character_children = self.utils.getchildren(character);


        let mut character_hash_map = HashMap::<String, usize>::new();
        for character_child in character_children  {
            let part_name = self.utils.get_name(character_child).unwrap_or_default();
            character_hash_map.insert(part_name, character_child);
        }

        Player {
            Username: player_name,
            Userid: player_userid,
            Character: character_hash_map,
            CharacterAddr: character,
            PlayerAddr: localplayer,
            utils: self.utils,
        }
    }
}