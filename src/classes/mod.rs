pub mod rbx {
    pub mod color3;
    pub mod vector2;
    pub mod vector3;
    #[allow(dead_code)]
    pub struct Quaternion {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
    #[allow(dead_code)]
    impl Quaternion { pub fn zero() -> Quaternion { Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 0.0 } } }
}

pub mod globals {
    pub mod globals;
}

pub mod cheat {
    pub mod offsets;
    pub mod player;
    #[allow(dead_code)]
    pub struct Matrix4 {
        pub data: [f32; 16]
    }
}