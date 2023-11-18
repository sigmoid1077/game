use bevy::ecs::component::Component;
use serde::{Deserialize, Serialize};
use std::net::{TcpStream, TcpListener};

pub(crate) mod client;
pub(crate) mod server;

pub(crate) const BUFFER_SIZE: usize = 1024;

#[derive(Component)]
pub(crate) struct ServerComponent(pub TcpStream);

#[derive(Component)]
pub(crate) struct ListenerComponent(pub TcpListener);

pub use client::{plugin::ClientPlugin, system_param::ClientSystemParam};

pub use server::{plugin::ServerPlugin, system_param::ServerSystemParam};

pub trait SendingPacket: Send + Serialize + Sync + 'static {
    fn serialize_packet(&self) -> Vec<u8>;
}

pub trait RecievingPacket: for<'de> Deserialize<'de> + Send + Sync + 'static {
    fn deserialize_packet(buffer: &[u8]) -> Self;
}
