use crate::utils::luau_env::game::register as game_register;
use crate::utils::luau_env::mem::register as mem_register;
use crate::utils::luau_env::offsets::register as offsets_register;

use mlua::Lua;

fn register(lua: &Lua) {
    game_register(lua);
}

pub fn state() -> Lua {
    let state = Lua::new();
    register(&state);
    mem_register(&state);
    offsets_register(&state);
    state
}