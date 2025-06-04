use mlua::{Function, Lua, String as LuaString, Number};
use crate::utils::cheat::*;
use crate::utils::luau_env::*;

fn find_first_child(lua: &Lua) -> Function {
    lua.create_function(| _, (address, object): (Number, LuaString) | {
        let address = address as usize;
        let object = object.to_string_lossy().to_string();

        let found_address = rbx::FindFirstChild(address, object);
        Ok(found_address)
    }).unwrap()
}

fn find_first_class(lua: &Lua) -> Function {
    lua.create_function(| _, (address, class): (Number, LuaString) | {
        let address = address as usize;
        let class = class.to_string_lossy().to_string();

        let found_address = rbx::FindFirstClass(address, class);
        Ok(found_address)
    }).unwrap()
}

fn get_name(lua: &Lua) -> Function {
    lua.create_function(| _, address: Number | {
        let address = address as usize;
        Ok(rbx::name(address))
    }).unwrap()
}

fn get_service(lua: &Lua) -> Function {
    lua.create_function(| _, class: LuaString | {
        let class = class.to_string_lossy().to_string();

        let found_address = rbx::GetService(class);
        Ok(found_address)
    }).unwrap()
}

fn localplayer(lua: &Lua) -> Function {
    lua.create_function( | lua: &Lua, _: () | {
        let mut player = rbx::GetLocalPlayer();
        let player_result = utils::player_to_lua(&mut player, lua);
        Ok(player_result)

    }).unwrap()
}

fn getplayers(lua: &Lua) -> Function {
    lua.create_function(| lua, _: () | {
        let players = lua.create_table().unwrap();
        let players_vec = rbx::GetPlayers();

        for mut player in players_vec {
            let player_binding = player.clone();
            players.set( player_binding.name, utils::player_to_lua(&mut player, lua) ).unwrap();
        };

        Ok(players)
    }).unwrap()
}

pub fn register(lua: &Lua) {
    let globals = lua.globals();
    let game = lua.create_table().unwrap();


    game.set("FindFirstChild", find_first_child(lua)).unwrap();
    game.set("FindFirstClass", find_first_class(lua)).unwrap();
    game.set("GetService", get_service(lua)).unwrap();
    game.set("LocalPlayer", localplayer(lua)).unwrap();
    game.set("GetPlayers", getplayers(lua)).unwrap();
    game.set("GetName", get_name(lua)).unwrap();

    globals.set("game", game).unwrap();
}
