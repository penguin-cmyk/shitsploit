use std::io::Error;
use memory_utils::process::Process;
use crate::offsets;
pub struct Utils<'a> {
    pub process: &'a Process,
}
#[allow(dead_code)]
impl<'a> Utils<'a> {
    pub fn new(process: &'a Process) -> Self { Self { process } }

    pub fn read_string(&self, address: usize) -> String {
        #[allow(unused)]
        let mut character = 0;
        let mut offset = 0;

        let mut result = Vec::new();

        while offset < 500 {
            character = self.process.read_memory::<u8>(address + offset).unwrap_or( 0);

            if character == 0 { // failed to read za memory / invalid
                break;
            }
            offset += 1;
            result.push(String::from_utf8_lossy(&[character]).to_string());
        }

        result.join("")

    }

    pub fn get_name(&self, address: usize) -> Option<String> {
        let name_ptr = self.process.read_memory::<usize>(address + offsets["Name"]).ok()?;
        let length = self.process.read_memory::<usize>(name_ptr + 0x10).ok()?;

        if length >= 16 {
            let name_ptr2 = self.process.read_memory::<usize>(name_ptr).ok()?;
            self.process.read_string(name_ptr2).ok()
        } else {
            self.process.read_string(name_ptr).ok()
        }
    }

    pub fn read_memory<T: Copy + Sized>(&self, address: usize) -> Result<T, Error> {
        self.process.read_memory::<T>(address)
    }

    pub fn write_memory<T: Copy + Sized>(&self, address: usize, value: &T) -> Result<(), Error> {
        self.process.write_memory::<T>(address, value)
    }

    pub fn getchildren(&self, address: usize) -> Vec<usize> {
        let mut return_vec: Vec<usize> = Vec::new();

        let start = self.process.read_memory::<usize>(address + offsets["Children"]).unwrap(); // Children
        let end = self.process.read_memory::<usize>(start + offsets["ChildrenEnd"]).unwrap_or(0); // ChildrenEnd

        if end == 0 { return return_vec; }

        let mut current = self.process.read_memory::<usize>(start).unwrap();

        while current < end {
            let child = self.process.read_memory::<usize>(current).unwrap();
            return_vec.push(child);
            current += 0x10;
        }

        return_vec
    }

    pub fn get_class(&self, address: usize) -> String {
        let class = self.process.read_memory::<usize>(address + offsets["ClassDescriptor"]).unwrap();
        let length = self.process.read_memory::<usize>(class + 0x10).unwrap();

        if length >= 16 {
            let class_ptr = self.process.read_memory::<usize>(class + 0x8).unwrap();
            self.process.read_string(class_ptr).ok().unwrap_or("".to_string())
        } else {
            self.process.read_string(class + 0x8).ok().unwrap_or("".to_string())

        }
    }

    pub fn find_first_child(&self, address: usize, name_to_find: String) -> usize {
        let children = self.getchildren(address);
        for child in children {
            let name = self.get_name(child).unwrap_or_default();
            if name_to_find == name {
                return child;
            }
        }
        0
    }

    pub fn walkspeed(&self, character: usize, speed: f32) {
        if character == 0 { return }

        let humanoid = self.find_first_child(character, "Humanoid".to_string());
        if humanoid == 0 { return }

        self.process.write_memory::<f32>(humanoid + offsets["WalkSpeed"], &speed).unwrap_or(());
        self.process.write_memory::<f32>(humanoid + offsets["WalkSpeedCheck"], &speed).unwrap_or(());
    }



}