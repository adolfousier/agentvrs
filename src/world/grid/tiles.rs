use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FloorKind {
    Wood,
    Tile,
    Carpet,
    Concrete,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WallKind {
    Solid,
    Window,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tile {
    Floor(FloorKind),
    Wall(WallKind),
    Desk,
    VendingMachine,
    CoffeeMachine,
    Couch,
    Plant,
    PinballMachine,
    GymTreadmill,
    WeightBench,
    YogaMat,
    FloorLamp,
    MeetingTable,
    SmallArmchair,
    Whiteboard,
    Rug,
    DoorOpen,
    KitchenCounter,
    ServerRack,
    FileCabinet,
}

impl Tile {
    pub fn is_solid(&self) -> bool {
        !matches!(self, Tile::Floor(_) | Tile::Rug | Tile::DoorOpen)
    }
}
