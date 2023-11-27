use bevy::ecs::component::Component;
use serde::{Deserialize, Serialize};
use std::net::{TcpStream, TcpListener};

pub mod client;
pub mod server;

#[derive(Component)]
pub(crate) struct ClientComponent(pub TcpStream);

#[derive(Component)]
pub(crate) struct ServerComponent(pub TcpListener);

pub trait Packet: for<'de> Deserialize<'de> + Send + Serialize + Sync + 'static {
    fn deserialize_packet(buffer: &[u8]) -> Self;
    fn serialize_packet(&self) -> Vec<u8>;
}
