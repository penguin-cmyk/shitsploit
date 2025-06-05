use std::clone::Clone;
use once_cell::sync::Lazy;

use crate::classes::cheat::offsets::Offsets;
use crate::classes::globals::globals::*;

fn rebase(address: usize) -> usize { base_address.clone() + address }

#[allow(non_upper_case_globals)]
pub static fake_datamodel_pointer: Lazy<usize> = Lazy::new(|| { rebase(0x6726238) });

#[allow(non_upper_case_globals)]
pub static TaskSchedulerPointer: Lazy<usize> = Lazy::new(|| { rebase(0x67E7748) });

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static offsets: Lazy<Offsets> = Lazy::new(|| Offsets {
    Sit:                    0x1DF,
    Team:                   0x258,
    Name:                   0x78,
    Parent:                 0x50,
    Health:                 0x19C,
    PlaceId:                0x1A0,
    UserId:                 0x278,
    Camera:                 0x3F8,
    FogColor:               0x104,
    Velocity:               0x14C,
    Position:               0x140,
    HipHeight:              0x1A8,
    JumpPower:              0x1B8,
    Children:               0x80,
    WalkSpeed:              0x1D8,
    CameraPos:              0x124,
    LocalPlayer:            0x128,
    PartSize:               0x2B0,
    Primitive:              0x178,
    CameraSubject:          0xF0,
    MousePosition:          0xF4,
    MoveDirection:          0x160,
    CanCollide:             0x313,
    ClassDescriptor:        0x18,
    ChildrenEnd:            0x8,
    OnDemandInstance:       0x30,
    WalkSpeedCheck:         0x3B0,
    ModelInstance:          0x330,
    MeshPartColor3:         0x1A8,
    FakeDataModelToData:    0x1B8,

    JobEnd:                 0x1D8,
    JobStart:               0x1D0,
    JobName:                0x18,

    Anchored:               0x311,
    CameraMode:             0x2C8,
    CameraType:             0x160,
    CanTouch:               0x314,
    DisplayName:            0x118,
    FOV:                    0x168,
    MaxHealth:              0x1BC,

    FakeDataModelPointer:   *fake_datamodel_pointer,
    TaskSchedulerPointer:   *TaskSchedulerPointer,
});
