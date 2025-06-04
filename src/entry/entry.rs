use crate::utils::luau_env::main::state;

pub fn entry() {

    let new_state = state();
    new_state.load(r#"
        local players: number = game.GetService("Players");
        print(players);

        local LocalPlayer: { [string]: number | string | (...any) -> (...any) } = game.LocalPlayer()
        table.foreach(LocalPlayer, print);

        local current_health = LocalPlayer.GetHealth();
        print("Health", current_health)

        --localplayer.SetHealth(0);

        local hrp = game.FindFirstChild(LocalPlayer.CharacterAddr, "HumanoidRootPart");
        local primtivie = mem.read_memory("usize", hrp + offsets.primitive);

        local players = game.GetPlayers();
        table.foreach(players, print);

        local position_offset = primtivie + offsets.Position;

        while true do
            local move_direction = LocalPlayer.MoveDirection();
            local current_pos = mem.read_memory("Vector3", position_offset);

            local new_position = {
                x = current_pos.x + move_direction.x / 30.0 ,
                y = current_pos.y + move_direction.y / 30.0,
                z = current_pos.z + move_direction.z / 30.0
            }

            mem.write_memory("Vector3", position_offset , new_position);
        end
    "#).exec().unwrap();

    /*
        let current_pos = local_player.get_position("HumanoidRootPart");
        println!("Current pos: {:?}", current_pos);
        local_player.set_position("HumanoidRootPart", Vector3::new(0.0, 0.0, 0.0));
    */
    // let players_service = GetService("Players".to_string());
    // println!("{}" , players_service);

}