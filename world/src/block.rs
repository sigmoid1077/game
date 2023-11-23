/*
 * // since there are only 192 block types (excluding shape and rotation), we can store the block
 * // type as a single byte
 * #[repr(u8)]
 * pub(crate) enum BlockVariant { ... }
 * 
 * pub(crate) struct ChunkBlockData {
 *     block_variant: BlockVariant,
 *     ...
 * }
 * 
 * impl ChunkBlockData {
 *     pub(crate) fn generate_block_mesh(...) {
 *         ...
 *     }
 * }
 * 
 * // write a function which takes a few parameters concerning information about how a specific
 * // block should look (for example divot size, divot frequency, divot color map, surface color
 * // map, etc.) and use the information, as well as the information about the surrounding blocks to
 * // randomly generate a mesh for a block. this approach to block rendering has many benefits. one
 * // benefit of this approach is that i, or any other person manually designing and creating the
 * // meshes for each block, each blocks several combinations of shapes and rotations, and each of 
 * // those's 3 variants, no longer has to do that, which saves loads of time, and allows more time
 * // to be spent working on core game logic, rather than the visual design aspect of the game. the
 * // second benefit to this approach is that each block of the same type is unique, which gives a
 * // more random and realistic feel to the game. to reiterate, not only are individial chunks
 * // procedurally generated, but the shape of each individual block is also procedurally generated.
 * 
 * // since the block meshes being generated from the above function have information about the
 * // surrounding blocks, the block meshes that are produced from that function will only contain
 * // the faces of the block that will be visible to the player (also known as culling). not only
 * // that but they will have the outline, similar to how blocks in terraria do.
 * 
 * // one possible (non?) issue to this approach is that when chunks are reloaded, blocks won't look
 * // the exact same as they were the last time the chunk was generated. this is because the this
 * // funciton uses random values to generate the mesh of the block, and those randomly generated
 * // values won't be the same every time the block is meshed. i'll have to further research how
 * // pseudo random number generation works, but as far as i know, using a simple seed for block
 * // generation, or even just using the same seed that the world uses, should clear up this issue.
 * 
 * // another possible issues that i might face when writing the above function is generating sloped
 * // blocks. will the bottom and top of the slope always be in the same position? where should
 * // divots be positioned? how exaggerated should divots be? will all of the blocks, including
 * // sloped blocks have the same rigid body geometry as the blocks? if not, does the appearance of
 * // sloped blocks, including the positioning and exaggeration of divots even matter?
 */
