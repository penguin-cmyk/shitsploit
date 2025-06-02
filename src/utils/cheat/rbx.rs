use crate::classes::rbx::{vector2::Vector2, vector3::Vector3, Quaternion };
use crate::classes::cheat::Matrix4;
use crate::classes::cheat::player::Player;
use crate::classes::globals::globals::*;
use crate::utils::cheat::mem;
use crate::offsets::offsets::offsets;

#[allow(dead_code)]
pub fn world_to_screen(
    world: Vector3,
    dimensions: Vector2,
    view_matrix: Matrix4
) -> Vector2 {
    let mut quaternion = Quaternion::zero();

    quaternion.x = (world.x * view_matrix.data[0]) + (world.y * view_matrix.data[1]) + (world.z * view_matrix.data[2]) + view_matrix.data[3];
    quaternion.y = (world.x * view_matrix.data[4]) + (world.y * view_matrix.data[5]) + (world.z * view_matrix.data[6]) + view_matrix.data[7];
    quaternion.z = (world.x * view_matrix.data[8]) + (world.y * view_matrix.data[9]) + (world.z * view_matrix.data[10]) + view_matrix.data[11];
    quaternion.w = (world.x * view_matrix.data[12]) + (world.y * view_matrix.data[13]) + (world.z * view_matrix.data[14]) + view_matrix.data[15];

    if quaternion.w < 0.1 { return Vector2::new(-1.0, -1.0) }

    let inv_w  = 1.0 / (quaternion.w);
    let mut new_vector = Vector3::zero();

    new_vector.x *= quaternion.x * inv_w;
    new_vector.y *= quaternion.y * inv_w;
    new_vector.z *= quaternion.z * inv_w;


    Vector2::new(
        (dimensions.x / 2.0 * new_vector.x) + (new_vector.x + dimensions.x / 2.0),
        -(dimensions.y / 2.0 * new_vector.y) + (new_vector.y + dimensions.y / 2.0)
    )
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn GetChildren(address: usize) -> Vec<usize> {
    let mut children = Vec::new();

    let start = process.read_memory::<usize>(address + offsets.Children).unwrap();
    let end = process.read_memory::<usize>(start + offsets.ChildrenEnd).unwrap_or(0);
    if end == 0 { return children }
    let mut current = process.read_memory::<usize>(start).unwrap();

    while current < end {
        let child = process.read_memory::<usize>(current).unwrap();
        children.push(child);
        current += 0x10;
    }
    children
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn FindFirstChild(address: usize, name_to_find: String) -> usize {
    let children = GetChildren(address);

    for child in children {
        let child_name = name(child);
        if name_to_find == child_name { return child }
    }
    0
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn GetService(name: String) -> usize {
    let datamodel = dm.lock().unwrap();

    return FindFirstChild(*datamodel, name)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn GetCameraPos() -> Vector3 {
    let workspace = GetService("Workspace".to_string());
    let camera = process.read_memory::<usize>(workspace + offsets.Camera).unwrap();

    let camera_pos = process.read_memory::<Vector3>( camera + offsets.CameraPos ).unwrap();
    camera_pos
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn MoveDirection(hum: usize) -> Vector3 {
    process.read_memory::<Vector3>( hum + offsets.MoveDirection ).unwrap()
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn GetClass(address: usize) -> String {
    let class = process.read_memory::<usize>(address + offsets.ClassDescriptor).unwrap();
    mem::read_string(class)
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn GetLocalPlayer() -> Player {
    let players =  GetService("Players".to_string());
    let player =  process.read_memory::<usize>(players + offsets.LocalPlayer).unwrap_or(0);
    if player == 0 { return Player::default() }

    let userid = process.read_memory::<usize>( player + offsets.UserId).unwrap();
    let name = name(player);

    let character_ptr = process.read_memory::<usize>( player + offsets.ModelInstance).unwrap_or(0);
    if character_ptr == 0 { return Player::default() }

    Player { name, userid, character: character_ptr, player }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn GetPlayers() -> Vec<Player> {
    let players = GetService("Players".to_string());
    let children = GetChildren(players);

    let mut players = Vec::<Player>::new();

    for player in children {
        let name = name(player);
        let userid = process.read_memory::<usize>(player + offsets.UserId).unwrap_or(0);

        if userid == 0 { continue }

        let character_ptr = process.read_memory::<usize>(player + offsets.ModelInstance).unwrap_or(0);
        if character_ptr == 0 { continue }

        players.push(Player { name, userid, character: character_ptr, player });
    };

    players
}

pub fn name(address: usize) -> String {
    let address = process.read_memory::<usize>(address + offsets.Name).unwrap_or(0);
    if address == 0 { return String::new() }
    mem::read_string(address)
}

pub fn datamodel() -> (usize, usize) {
    let fake_dm = process.read_memory::<usize>(offsets.FakeDataModelPointer).unwrap();
    let real_dm = process.read_memory::<usize>(fake_dm + offsets.FakeDataModelToData).unwrap();

    {
        let mut datamodel = dm.lock().unwrap();
        *datamodel = real_dm;
    }

    (real_dm, fake_dm)
}