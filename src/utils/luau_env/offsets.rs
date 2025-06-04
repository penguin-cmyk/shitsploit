use mlua::{ Lua, Table};
use mlua::prelude::LuaString;
use crate::offsets::offsets::offsets;
pub fn register(lua: &Lua) {
    let globals = lua.globals();
    let offsets_t = lua.create_table().unwrap();

    let mt = lua.create_table().unwrap();
    let __index = lua.create_function(|_, (_, key): (Table, LuaString) | {
            let key = key.to_string_lossy();

            let value = match key.to_lowercase().as_str() {
                "sit"                  => offsets.Sit,
                "team"                 => offsets.Team,
                "name"                 => offsets.Name,
                "parent"               => offsets.Parent,
                "health"               => offsets.Health,
                "placeid"              => offsets.PlaceId,
                "userid"               => offsets.UserId,
                "camera"               => offsets.Camera,
                "fogcolor"             => offsets.FogColor,
                "velocity"             => offsets.Velocity,
                "position"             => offsets.Position,
                "hipheight"            => offsets.HipHeight,
                "jumppower"            => offsets.JumpPower,
                "children"             => offsets.Children,
                "walkspeed"            => offsets.WalkSpeed,
                "camerapos"            => offsets.CameraPos,
                "localplayer"          => offsets.LocalPlayer,
                "partsize"             => offsets.PartSize,
                "primitive"            => offsets.Primitive,
                "camerasubject"        => offsets.CameraSubject,
                "mouseposition"        => offsets.MousePosition,
                "movedirection"        => offsets.MoveDirection,
                "cancollide"           => offsets.CanCollide,
                "classdescriptor"      => offsets.ClassDescriptor,
                "childrenend"          => offsets.ChildrenEnd,
                "ondemandinstance"     => offsets.OnDemandInstance,
                "walkspeedcheck"       => offsets.WalkSpeedCheck,
                "modelinstance"        => offsets.ModelInstance,
                "meshpartcolor3"       => offsets.MeshPartColor3,
                "fakedatamodeltodata"  => offsets.FakeDataModelToData,
                "fakedatamodelpointer" => offsets.FakeDataModelPointer,
                _ => return Err(mlua::Error::RuntimeError(format!("Unknown offset: {}", key))),
            };

            Ok(value)
    }).unwrap();
    mt.set("__index", __index).unwrap();
    offsets_t.set_metatable(Some(mt));
    offsets_t.set_readonly(true);

    globals.set("offsets", offsets_t).unwrap();
}