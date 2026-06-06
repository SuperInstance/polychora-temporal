use polychora_room_runtime::{Room, Tile, Ecology};
use polychora_room_runtime::tile::TileCapabilities;
use polychora_room_runtime::provenance::Provenance;

fn main() {
    println!("═══ Ecology Demo ═══\n");

    let mut main_room = Room::new("forge-alpha");
    main_room.add_agent("ensign-7");
    main_room.add_agent("ensign-12");

    let mut forge_tile = Tile::new();
    forge_tile.add_preset("ternary-forge: default");
    forge_tile.add_preset("temperature: 1200K");
    let mut forge_prov = Provenance::new("oracle2");
    forge_prov.record("created forge tile with ternary forge preset", "oracle2");
    assert!(!forge_prov.is_empty());

    let mut research_tile = Tile::new();
    research_tile.add_preset("ternary-research: depth-analysis");
    let mut research_prov = Provenance::new("oracle2");
    research_prov.record("created research tile", "oracle2");

    main_room.add_tile(forge_tile);
    main_room.add_tile(research_tile);
    assert_eq!(main_room.tiles.len(), 2);
    println!("✓ Room '{}' created with {} tiles and {} agents",
        main_room.scope, main_room.tiles.len(), main_room.agents.len());

    let mut ecology = Ecology::new();
    ecology.add_room(main_room);
    let split = ecology.split_room("forge-alpha", 1);
    assert!(split.is_some());
    println!("✓ Room split: forge-alpha.a + forge-alpha.b");

    let merged = ecology.merge_rooms("forge-alpha.a", "forge-alpha.b");
    assert!(merged.is_some());
    let merged = merged.unwrap();
    assert_eq!(merged.tiles.len(), 2);
    println!("✓ Rooms merged into '{}'", merged.scope);

    let mut freeze_room = Room::new("freeze-test");
    freeze_room.freeze_context(b"{\"state\": \"snapshot\"}".to_vec());
    assert!(freeze_room.is_frozen());
    println!("✓ Context frozen: {} bytes", freeze_room.frozen_context.len());

    let temporal_caps = TileCapabilities { host_agents: true, temporal: true, persistent: true, emit_events: true };
    let mut temporal_tile = Tile::new();
    temporal_tile = temporal_tile.with_capabilities(temporal_caps);
    assert!(temporal_tile.capabilities.temporal);
    println!("✓ Temporal tile with full capabilities");

    println!("\n═══ All ecology tests passed ═══");
}
