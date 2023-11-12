use bevy::ecs::component::Component;
use serde::{Deserialize, Serialize};
use std::net::{TcpStream, TcpListener};

pub const BUFFER_SIZE: usize = 1024;

#[derive(Component)]
pub struct StreamComponent(pub TcpStream);

#[derive(Component)]
pub struct ListenerComponent(pub TcpListener);

pub trait SendingPacket: Send + Serialize + Sync + 'static {
    fn serialize_packet(&self) -> Vec<u8>;
}

pub trait RecievingPacket: for<'de> Deserialize<'de> + Send + Sync + 'static {
    fn deserialize_packet(buffer: &[u8]) -> Self;
}