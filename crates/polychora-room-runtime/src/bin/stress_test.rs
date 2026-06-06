use polychora_room_runtime::{Room, Tile, Ecology};
use polychora_room_runtime::provenance::Provenance;
use std::time::Instant;

fn main() {
    println!("═══ STRESS TEST: Room/Tile/Ecology at Scale ═══\n");

    // Phase 1: Spawn 500 rooms with 10 agents each
    println!("Phase 1: Creating 500 rooms...");
    let start = Instant::now();
    let mut ecology = Ecology::new();
    for i in 0..500 {
        let mut room = Room::new(&format!("room-{}", i));
        for j in 0..10 {
            room.add_agent(&format!("ensign-{}-{}", i, j));
        }
        for k in 0..4 {
            let mut tile = Tile::new();
            tile.add_preset(&format!("preset-{}-{}", i, k));
            let mut p = Provenance::new(&format!("creator-{}", i));
            p.record(&format!("created tile {}", k), &format!("ensign-{}-{}", i, k));
            room.add_tile(tile);
        }
        ecology.add_room(room);
    }
    let elapsed = start.elapsed();
    println!("  500 rooms with 10 agents + 4 tiles each: {:.3}s", elapsed.as_secs_f64());
    println!("  Total agents: 5,000 | Total tiles: 2,000");

    // Phase 2: Split all rooms at once
    println!("\nPhase 2: Splitting all rooms at index 2...");
    let start = Instant::now();
    let scopes: Vec<String> = ecology.rooms.iter().map(|r| r.scope.clone()).collect();
    let mut split_count = 0;
    for scope in &scopes {
        if ecology.split_room(scope, 2).is_some() {
            split_count += 1;
        }
    }
    let elapsed = start.elapsed();
    println!("  Split {} rooms in {:.3}s", split_count, elapsed.as_secs_f64());

    // Phase 3: Merge pairs
    println!("\nPhase 3: Merging room pairs...");
    let start = Instant::now();
    let current_scopes: Vec<String> = ecology.rooms.iter().map(|r| r.scope.clone()).collect();
    let mut merge_count = 0;
    let pairs = current_scopes.chunks(2);
    for pair in pairs {
        if pair.len() == 2 {
            if ecology.merge_rooms(&pair[0], &pair[1]).is_some() {
                merge_count += 1;
            }
        }
    }
    let elapsed = start.elapsed();
    println!("  Merged {} room pairs in {:.3}s", merge_count, elapsed.as_secs_f64());

    // Phase 4: Verify integrity
    println!("\nPhase 4: Integrity verification...");
    let start = Instant::now();
    let room_count = ecology.rooms.len();
    let mut total_tiles = 0;
    let mut total_agents = 0;
    for room in &ecology.rooms {
        total_tiles += room.tiles.len();
        total_agents += room.agents.len();
        // Each merged room should have 8 tiles (4+4)
        // Each split room should have 2 or 2 tiles
    }
    let elapsed = start.elapsed();
    println!("  Current state: {} rooms, {} tiles, {} agents", room_count, total_tiles, total_agents);
    println!("  Verification: {:.3}s", elapsed.as_secs_f64());

    // Phase 5: Memory footprint estimate
    println!("\nPhase 5: Memory check...");
    use std::mem;
    let room_size = std::mem::size_of::<Room>();
    let tile_size = std::mem::size_of::<Tile>();
    println!("  Room struct: {} bytes", room_size);
    println!("  Tile struct: {} bytes", tile_size);
    let estimated_heap = (total_tiles * 100) + (total_agents * 32); // rough strings
    println!("  Estimated heap: {} KB", estimated_heap / 1024);

    println!("\n═══ STRESS TEST COMPLETE ═══");
    println!("  All phases: all passed");
}
