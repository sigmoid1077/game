use bevy::{prelude::*, utils::hashbrown::HashMap};
use crate::chunk::ChunkData;

pub(crate) struct ChunkMap(HashMap<[u8; 3], ChunkData>);
