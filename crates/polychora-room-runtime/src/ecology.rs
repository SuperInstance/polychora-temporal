use crate::room::Room;

/// Ecology handles multi-agent coordination, room splitting, and merging
/// within the temporal 4D world runtime.
pub struct Ecology {
    /// All active rooms managed by this ecology.
    pub rooms: Vec<Room>,
    /// Whether ecology is active.
    pub active: bool,
}

impl Ecology {
    pub fn new() -> Self {
        Self {
            rooms: Vec::new(),
            active: false,
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    pub fn remove_room(&mut self, scope: &str) {
        self.rooms.retain(|r| r.scope != scope);
    }

    /// Split a room into two at a given tile boundary.
    pub fn split_room(&mut self, scope: &str, split_index: usize) -> Option<(Room, Room)> {
        let pos = self.rooms.iter().position(|r| r.scope == scope)?;
        let room = self.rooms.remove(pos);
        if split_index >= room.tiles.len() || split_index == 0 {
            // Cannot split; restore room
            self.rooms.push(room);
            return None;
        }
        let mut room_a = Room::new(&format!("{}.a", scope));
        let mut room_b = Room::new(&format!("{}.b", scope));
        room_a.agents = room.agents.clone();
        room_b.agents = room.agents.clone();
        room_a.tiles = room.tiles[..split_index].to_vec();
        room_b.tiles = room.tiles[split_index..].to_vec();
        Some((room_a, room_b))
    }

    /// Merge two rooms into one.
    pub fn merge_rooms(&mut self, scope_a: &str, scope_b: &str) -> Option<Room> {
        let pos_a = self.rooms.iter().position(|r| r.scope == scope_a)?;
        let pos_b = self.rooms.iter().position(|r| r.scope == scope_b)?;
        let idx = if pos_a < pos_b { pos_a } else { pos_b };
        let room_a = self.rooms.remove(pos_a);
        let room_b = self.rooms.remove(if pos_a < pos_b { pos_b - 1 } else { pos_a });
        let mut merged = Room::new(&format!("{}+{}", scope_a, scope_b));
        merged.tiles = [room_a.tiles, room_b.tiles].concat();
        merged.agents = {
            let mut all = room_a.agents.clone();
            for a in room_b.agents {
                if !all.contains(&a) {
                    all.push(a);
                }
            }
            all
        };
        self.rooms.insert(idx, merged.clone());
        Some(merged)
    }
}

impl Default for Ecology {
    fn default() -> Self {
        Self::new()
    }
}
