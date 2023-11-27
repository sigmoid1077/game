
pub struct ChunkVoxelData {
    voxel_type: VoxelType,
    _voxel_modification: Option<VoxelModification>
}

#[derive(PartialEq, Default)]
pub enum VoxelType {
    #[default]
    Air,
    Dirt,
    Stone,
    /* ... */
}

pub struct VoxelModification {
    voxel_variant: VoxelVariant,
    voxel_rotation: VoxelRotation
}

#[derive(Default)]
pub enum VoxelVariant {
    #[default]
    Full,
    Slope,
    Half,
    /* ... */
}

#[derive(Default)]
pub struct VoxelRotation;
