use common::semantics::Semantics4D;
use polychora::content_registry;
use polychora::shared::voxel::BlockData;

/// An EventVoxel maps an occurrence to a voxel representation.
pub struct EventVoxel {
    /// The block data representing this event.
    pub block: BlockData,
    /// The spatial coordinates (X, Y, Z).
    pub spatial_coords: [i64; 3],
    /// The temporal W coordinate.
    pub temporal_w: u64,
}

impl EventVoxel {
    pub fn new(x: i64, y: i64, z: i64, w: u64, block: BlockData) -> Self {
        Self {
            block,
            spatial_coords: [x, y, z],
            temporal_w: w,
        }
    }

    /// Converts this event voxel's data into world coordinates
    /// using the given semantics for W-axis interpretation.
    pub fn to_world_coords(&self, _semantics: Semantics4D) -> [i64; 4] {
        [
            self.spatial_coords[0],
            self.spatial_coords[1],
            self.spatial_coords[2],
            self.temporal_w as i64,
        ]
    }
}

/// Map an event material token into a voxel block representation.
///
/// Uses the polychora content registry's static token-to-block mapping.
/// Token 0 (air) maps to `BlockData::AIR`, and tokens 1–68 map to their
/// corresponding block types from the polychora-content plugin registration
/// order.
pub fn event_to_block(material_token: u8) -> BlockData {
    content_registry::block_data_from_material_token(material_token)
}
