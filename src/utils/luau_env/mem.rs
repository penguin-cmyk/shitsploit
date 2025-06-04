use mlua::{Lua, Function, Number, Value, String as LuaString};
use crate::utils::luau_env::utils::*;
use crate::utils::cheat::mem;

pub fn read_memory(lua: &Lua) -> Function {
    lua.create_function(|lua: &Lua, (type_str, address): (LuaString, Number) | {
        let result = read_mem(lua, type_str.to_string_lossy().as_str(), address as usize);
        Ok(result)
    }).unwrap()
}

pub fn read_string(lua: &Lua) -> Function {
    lua.create_function(|_lua: &Lua, address: Number | {
        let res = mem::read_string(address as usize, false);
        Ok(res)
    }).unwrap()
}
pub fn write_memory(lua: &Lua) -> Function {
    lua.create_function(|_lua: &Lua, (type_str, address, value): (LuaString, Number, Value)| {
        let type_str = string_to_type(type_str.to_string_lossy().as_str());
        let address = address as usize;

        write_mem(address, type_str, value);

        Ok(())
    }).unwrap()
}


pub fn register(lua: &Lua) {
    let globals = lua.globals();
    let mem = lua.create_table().unwrap();

    mem.set("read_string", read_string(lua)).unwrap();
    mem.set("read_memory", read_memory(lua)).unwrap();
    mem.set("write_memory", write_memory(lua)).unwrap();
    globals.set("mem", mem).unwrap();
}