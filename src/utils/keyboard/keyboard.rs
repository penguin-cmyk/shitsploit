use device_query::{ DeviceState, DeviceQuery, Keycode };

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn KeyDown(keycode: &Keycode) -> bool {
    let state = DeviceState::new();
    state.get_keys().contains(&keycode)
}
