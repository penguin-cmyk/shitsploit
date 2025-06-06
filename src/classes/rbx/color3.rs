#[derive(Clone, Copy, Debug)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Color3 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
#[allow(dead_code)]

impl Color3 {
    pub fn new(r: f32, g: f32, b: f32) -> Self { Color3 { r, g, b } }
}