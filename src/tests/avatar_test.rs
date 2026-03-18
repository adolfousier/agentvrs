use crate::avatar::furniture::furniture_sprite;
use crate::avatar::sprite::empty_frame;
use crate::world::Tile;

/// Every furniture tile must produce a non-empty sprite (not the fallback).
#[test]
fn all_furniture_tiles_have_sprites() {
    let furniture_tiles = [
        Tile::Desk,
        Tile::VendingMachine,
        Tile::CoffeeMachine,
        Tile::Couch,
        Tile::Plant,
        Tile::PinballMachine,
        Tile::GymTreadmill,
        Tile::Whiteboard,
        Tile::WeightBench,
        Tile::YogaMat,
        Tile::FloorLamp,
        Tile::MeetingTable,
        Tile::SmallArmchair,
        Tile::ServerRack,
        Tile::FileCabinet,
        Tile::KitchenCounter,
    ];

    let empty = empty_frame();

    for tile in &furniture_tiles {
        let sprite = furniture_sprite(tile);
        // At least one cell must differ from the empty frame
        let has_content = sprite.iter().enumerate().any(|(r, row)| {
            row.iter()
                .enumerate()
                .any(|(c, cell)| cell.ch != empty[r][c].ch || cell.bg != empty[r][c].bg)
        });
        assert!(
            has_content,
            "{tile:?} returned an empty sprite — add a sprite for it"
        );
    }
}

/// Sprite dimensions: every furniture sprite is 4 wide x 3 tall.
#[test]
fn furniture_sprites_correct_dimensions() {
    let tiles = [
        Tile::WeightBench,
        Tile::YogaMat,
        Tile::FloorLamp,
        Tile::MeetingTable,
        Tile::SmallArmchair,
        Tile::ServerRack,
        Tile::FileCabinet,
        Tile::KitchenCounter,
    ];

    for tile in &tiles {
        let sprite = furniture_sprite(tile);
        assert_eq!(sprite.len(), 3, "{tile:?} sprite should have 3 rows");
        for (i, row) in sprite.iter().enumerate() {
            assert_eq!(row.len(), 4, "{tile:?} row {i} should have 4 columns");
        }
    }
}

/// Server rack should have green LED indicators.
#[test]
fn server_rack_has_leds() {
    let sprite = furniture_sprite(&Tile::ServerRack);
    // Row 1 (middle) should have green LED cells
    let has_green = sprite[1]
        .iter()
        .any(|cell| matches!(cell.fg, ratatui::style::Color::Rgb(0, 255, 100)));
    assert!(has_green, "Server rack should have green LED indicators");
}
