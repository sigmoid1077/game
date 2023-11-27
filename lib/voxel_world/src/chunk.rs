use bevy::prelude::*;
use crate::voxel::ChunkVoxelData;
use ndshape::{ConstShape, ConstShape3u16};

pub(crate) const CHUNK_SIZE: u16 = 32;
pub(crate) type ChunkShape: = ConstShape3u16<CHUNK_SIZE, CHUNK_SIZE,CHUNK_SIZE>;
pub(crate) type VoxelData = [ChunkVoxelData; ChunkShape::SIZE as usize];

pub(crate) struct ChunkData {
    voxel_data: VoxelData,
    entity: Entity
}
