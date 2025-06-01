use std::collections::HashMap;
use once_cell::sync::Lazy;
#[allow(non_upper_case_globals)]
pub static offsets: Lazy<HashMap<String, usize>> = Lazy::new(|| {
    HashMap::from([
        ("Primitive".to_string(), 0x178),
        ("Position".to_string(), 0x140),
        ("Velocity".to_string(), 0x14C),
        ("ModelInstance".to_string(), 0x330),
        ("UserId".to_string(), 0x278),
        ("LocalPlayer".to_string(), 0x128),
        ("Name".to_string(), 0x78),
        ("Children".to_string(), 0x80),
        ("ChildrenEnd".to_string(), 0x8),
        ("WalkSpeed".to_string(), 0x1D8),
        ("WalkSpeedCheck".to_string(), 0x3B0),
        ("ClassDescriptor".to_string(), 0x18),
        ("Parent".to_string(), 0x50),
        ("OnDemandInstance".to_string(), 0x30),
        ("MoveDirection".to_string(), 0x160),
        ("CameraSubject".to_string(), 0xF0),
        ("MousePosition".to_string(), 0xF4),
        ("PartSize".to_string(), 0x2B0),
        ("Sit".to_string(), 0x1DF),
        ("MeshPartColor3".to_string(), 0x1A0),
        ("Health".to_string(), 0x19C),
        ("HipHeight".to_string(), 0x1A8),
        ("FogColor".to_string(), 0x104),
        ("CanCollide".to_string(), 0x313),
        ("Team".to_string(), 0x258),
        ("JumpPower".to_string(), 0x1B8),
        ("PlaceId".to_string(), 0x1A0)
    ])
});