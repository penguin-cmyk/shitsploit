use crate::classes::globals::globals::*;

#[allow(dead_code)]
fn string(address: usize) -> String {
    let mut character = 0;
    let mut offset = 0;

    let mut result = Vec::new();

    while offset < 400 {
        character = process.read_memory::<u8>(address + offset).unwrap_or(0);
        if character == 0 { break }

        offset += 1;
        result.push(String::from_utf8_lossy(&[character]).to_string());
    };

    result.join("")
}

pub fn read_string(address: usize) -> String {
    let length = process.read_memory::<usize>(address + 0x10).unwrap_or(0);

    if length >= 16 { string(process.read_memory::<usize>(address).unwrap_or(0)) }
    else { string(address) }
}
