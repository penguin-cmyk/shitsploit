use mlua::{Lua, String as LuaString, Number, Table, Value, IntoLua};
use mlua::prelude::LuaResult;
use crate::classes::cheat::player::Player;
use crate::classes::globals::globals::process;
use crate::classes::rbx::vector3::Vector3;

#[derive(Debug)]
pub enum Types {
    Usize,
    F32,
    F64,
    U8,
    U16,
    Vector3,
    U32,
    U64
}


pub fn string_to_type(s: &str) -> Option<Types> {
    match s.to_lowercase().as_str() {
        "usize" => Some(Types::Usize),
        "f32" => Some(Types::F32),
        "f64" => Some(Types::F64),
        "u8" => Some(Types::U8),
        "u16" => Some(Types::U16),
        "vector3" => Some(Types::Vector3),
        "u32" => Some(Types::U32),
        "u64" => Some(Types::U64),
        _ => None
    }
}

pub fn write_mem(address: usize, type_str: Option<Types>, value: Value) {
    match type_str {
        Some(Types::Usize) => {
            if let Value::Integer(i) = value {
                let val = i as usize;
                process.write_memory::<usize>(address, &val).unwrap();
            }
        }
        Some(Types::U64) => {
            if let Value::Integer(i) = value {
                let val = i as u64;
                process.write_memory::<u64>(address, &val).unwrap();
            }
        }
        Some(Types::U32) => {
            if let Value::Integer(i) = value {
                let val = i as u32;
                process.write_memory::<u32>(address, &val).unwrap();
            }
        }
        Some(Types::U16) => {
            if let Value::Integer(i) = value {
                let val = i as u16;
                process.write_memory::<u16>(address, &val).unwrap();
            }
        }
        Some(Types::U8) => {
            if let Value::Integer(i) = value {
                let val = i as u8;
                process.write_memory::<u8>(address, &val).unwrap();
            }
        }
        Some(Types::F32) => {
            match value {
                Value::Number(n) => {
                    process.write_memory::<f32>(address, &(n as f32)).unwrap();
                }
                Value::Integer(i) => {
                    process.write_memory::<f32>(address, &((i as f32))).unwrap();
                }
                _ => eprintln!("Expected number for f32, got {:?}", value),
            }
        }
        Some(Types::F64) => {
            match value {
                Value::Number(n) => {
                    process.write_memory::<f64>(address, &(n as f64)).unwrap();
                }
                Value::Integer(i) => {
                    process.write_memory::<f64>(address, &((i as f64))).unwrap();
                }
                _ => eprintln!("Expected number for f64, got {:?}", value),
            }
        }
        Some(Types::Vector3) => {
            if let Value::Table(table) = value {
                let vector_3 = table_to_vector3(table);
                process.write_memory::<Vector3>(address, &vector_3).unwrap();
            } else {
                eprintln!("Expected table for Vector3, got {:?}", value);
            }
        }
        _ => {}
    }
}

pub fn read_mem(lua: &Lua, typename: &str, address: usize) -> LuaResult<Value> {
    match string_to_type(typename) {
        Some(Types::Usize) => {
            let value = process.read_memory::<usize>(address)?;
            value.into_lua(lua)
        }
        Some(Types::F32) => {
            let value = process.read_memory::<f32>(address)?;
            value.into_lua(lua)
        }
        Some(Types::F64) => {
            let value = process.read_memory::<f64>(address)?;
            value.into_lua(lua)
        }

        Some(Types::U8) => {
            let value = process.read_memory::<u8>(address)?;
            value.into_lua(lua)
        }

        Some(Types::U16) => {
            let value = process.read_memory::<u16>(address)?;
            value.into_lua(lua)
        }
        Some(Types::U32) => {
            let value = process.read_memory::<u32>(address)?;
            value.into_lua(lua)
        }

        Some(Types::U64) => {
            let value = process.read_memory::<u64>(address)?;
            value.into_lua(lua)
        }

        Some(Types::Vector3) => {
            let vec = process.read_memory::<Vector3>(address)?;
            let table = lua.create_table()?;
            table.set("x", vec.x)?;
            table.set("y", vec.y)?;
            table.set("z", vec.z)?;

            table.into_lua(lua)
        },

        None => ("".to_string()).into_lua(lua),
    }
}


pub fn table_to_vector3(table: Table) -> Vector3 {
    let x = table.get::<Number>("x").unwrap() as f32;
    let y = table.get::<Number>("y").unwrap() as f32;
    let z = table.get::<Number>("z").unwrap() as f32;

    Vector3::new(x, y, z)
}

pub fn vector3_to_table(lua: &Lua, pos: Vector3) -> Table {
    let pos_result = lua.create_table().unwrap();
    pos_result.set("x", pos.x).unwrap();
    pos_result.set("y", pos.y).unwrap();
    pos_result.set("z", pos.z).unwrap();

    pos_result
}

pub fn player_to_lua(player: &mut Player, lua: &Lua) -> Table {
    let result = lua.create_table().unwrap();
    let player_binding = player.clone();
    result.set("Name", player_binding.name).unwrap();
    result.set("UserId", player_binding.userid).unwrap();
    result.set("CharacterAddr", player_binding.character).unwrap();
    result.set("PlayerAddr", player_binding.player).unwrap();

    let player_binding = player.clone();
    let health_fn = lua.create_function(move |_, _: () | {
        let health = player_binding.health();
        Ok(health)
    }).unwrap();
    result.set("GetHealth", health_fn).unwrap();

    let player_binding = player.clone();
    let team_fn = lua.create_function(move |_, _: () | {
        let team = player_binding.team();
        Ok(team)
    }).unwrap();
    result.set("GetTeam", team_fn).unwrap();

    let player_binding = player.clone();
    let set_health = lua.create_function(move |_, health: Number | {
        let health = health as f32;
        player_binding.set_health(health);
        Ok(())
    }).unwrap();
    result.set("SetHealth", set_health).unwrap();

    let player_binding = player.clone();
    let get_position = lua.create_function(move |lua: &Lua, part: LuaString | {
        let pos = player_binding.get_position(part.to_string_lossy().as_str());

        Ok(vector3_to_table(lua, pos))
    }).unwrap();
    result.set("GetPosition", get_position).unwrap();

    let player_binding = player.clone();
    let set_position = lua.create_function(move |_lua: &Lua, (part, pos): (LuaString, Table) | {
        player_binding.set_position(part.to_string_lossy().as_str(), table_to_vector3(pos));
        Ok(())
    }).unwrap();
    result.set("SetPosition", set_position).unwrap();

    let player_binding = player.clone();
    let move_direction = lua.create_function(move |lua: &Lua, _: () | {
        let table = vector3_to_table(lua, player_binding.move_direction());
        Ok(table)
    }).unwrap();
    result.set("MoveDirection", move_direction).unwrap();

    let player_binding = player.clone();
    let get_velocity = lua.create_function(move |lua: &Lua, part: LuaString | {
        let table = vector3_to_table(lua, player_binding.get_velocity(part.to_string_lossy().as_str()));
        Ok(table)
    }).unwrap();
    result.set("GetVelocity", get_velocity).unwrap();

    let player_binding = player.clone();
    let set_velocity = lua.create_function(move |_lua: &Lua, (part, velocity): (LuaString, Table) | {
        player_binding.set_velocity(part.to_string_lossy().as_str(), table_to_vector3(velocity));
        Ok(())
    }).unwrap();
    result.set("SetPosition", set_velocity).unwrap();


    result
}
