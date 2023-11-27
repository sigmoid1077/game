use bevy::{prelude::*, ecs::system::SystemParam};

use crate::voxel::ChunkVoxelData;

#[derive(SystemParam)]
pub struct VoxelWorldSystemParam;

impl VoxelWorldSystemParam {
    pub fn load_chunk(&self, /* chunk coordinate position */ chunk_voxel_data: ChunkVoxelData) {
        //! Check if chunk is saved in the chunk cache. If it is, load the chunk.
        //! If it's not, build the chunk mesh from the chunk's voxel data,
        //! load the chunk, then lastly save the chunk mesh in the chunk cache.
        //! Note that a chunk's voxel data is built from the chunk generator.
        
        todo!();
    }

    pub fn unload_chunk(&self) {
        todo!();
    }

    pub fn set_voxel(&self) {
        todo!();
    }
}
