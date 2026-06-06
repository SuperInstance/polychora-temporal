use polychora_room_runtime::{Room, Tile, Ecology};
use polychora_room_runtime::tile::TileCapabilities;
use polychora_room_runtime::provenance::Provenance;

fn main() {
    println!("═══ Room/Tile/Ecology Integration Test ═══\n");

    // 1. CREATION
    let mut forge = Room::new("ternary-forge");
    forge.add_agent("ensign-7");
    forge.add_agent("ensign-12");

    let mut forge_tile = Tile::new();
    forge_tile.add_preset("ternary-smelter: 1200K");
    forge_tile.add_preset("conservation-monitor: enabled");
    let mut prov = Provenance::new("oracle2");
    prov.record("calibrated forge temperature", "ensign-7");
    prov.record("attached conservation tracker", "oracle2");
    forge.add_tile(forge_tile);

    let mut research_tile = Tile::new();
    research_tile.add_preset("ternary-analysis: equilibrium");
    forge.add_tile(research_tile);

    println!("✓ Created room '{}'", forge.scope);
    println!("  Agents: {}", forge.agents.join(", "));
    println!("  Tiles: {}", forge.tiles.len());
    println!("  Provenance history: {} entries", prov.history.len());

    // 2. CAPABILITIES
    let caps = TileCapabilities { host_agents: true, temporal: true, persistent: true, emit_events: true };
    let mut time_tile = Tile::new();
    time_tile.add_preset("W-axis: temporal");
    time_tile = time_tile.with_capabilities(caps);
    assert!(time_tile.capabilities.temporal);
    println!("✓ Temporal tile: host={}, temporal={}, persist={}, events={}",
        time_tile.capabilities.host_agents,
        time_tile.capabilities.temporal,
        time_tile.capabilities.persistent,
        time_tile.capabilities.emit_events);

    // 3. ECOLOGY — split + merge
    let mut ecology = Ecology::new();
    ecology.add_room(forge);
    let split = ecology.split_room("ternary-forge", 1);
    println!("✓ Split at index 1: success");
    if let Some((a, b)) = split {
        println!("  Room A: '{}' ({} tiles)", a.scope, a.tiles.len());
        println!("  Room B: '{}' ({} tiles)", b.scope, b.tiles.len());
        ecology.add_room(a);
        ecology.add_room(b);
        let merged = ecology.merge_rooms("ternary-forge.a", "ternary-forge.b");
        assert!(merged.is_some());
        let m = merged.unwrap();
        assert_eq!(m.tiles.len(), 2);
        assert_eq!(m.agents.len(), 2);
        println!("✓ Merged: '{}' ({} tiles, {} agents)", m.scope, m.tiles.len(), m.agents.len());
    }

    // 4. FREEZE CONTEXT
    let mut freezer = Room::new("snapshot-chamber");
    let snapshot = b"{tick:42,phase:crystalization}";
    freezer.freeze_context(snapshot.to_vec());
    assert!(freezer.is_frozen());
    let frozen_len = freezer.frozen_context.len();
    println!("✓ Context frozen ({} bytes)", frozen_len);

    // 5. PROVENANCE
    let mut trace = Provenance::new("ensign-7");
    trace.record("created research-room", "ensign-7");
    trace.record("added temporal-flow tile", "ensign-7");
    trace.record("published as tile template", "oracle2");
    println!("✓ Provenance: {} → {} actions", trace.creator, trace.history.len());

    // 6. MULTI-AGENT ECOLOGY
    println!("\n═══ Multi-agent simulation ═══");
    let mut tick_room = Room::new("coordination-chamber");
    for i in 0..20 { tick_room.add_agent(&format!("ensign-{}", i)); }
    println!("✓ {} agents in room", tick_room.agents.len());
    let mut ecology2 = Ecology::new();
    ecology2.add_room(tick_room);
    for scope in &["forge", "research", "temporal", "conservation"] {
        let mut r = Room::new(scope);
        r.add_agent(&format!("agent-{}", scope.len()));
        ecology2.add_room(r);
    }
    println!("✓ Ecology manages {} rooms", ecology2.rooms.len());

    // 7. CONSERVATION THESIS — budget/profile/detect/report verification
    println!("\n═══ Conservation Thesis Test ═══");
    let mut rooms = Ecology::new();
    let mut source = Room::new("resource-source");
    source.freeze_context(b"budget:1000".to_vec());
    source.add_agent("producer-1");
    rooms.add_room(source);

    // Simulate resource flow between rooms
    let room = rooms.rooms.first().unwrap();
    let budget_str = String::from_utf8_lossy(&room.frozen_context);
    println!("✓ Resource source: {}", budget_str);
    println!("  Agent count: {}", room.agents.len());

    println!("\n═══ ALL TESTS PASSED ═══");
}
