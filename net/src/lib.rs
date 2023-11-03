pub mod server;
pub mod client;

use bevy::ecs::component::Component;
use serde::{Deserialize, Serialize};
use std::net::{TcpStream, TcpListener};

const BUFFER_SIZE: usize = 1024;

#[derive(Component)]
struct StreamComponent(pub TcpStream);

#[derive(Component)]
struct ListenerComponent(pub TcpListener);

pub use client::ClientPlugin;

pub use server::ServerPlugin;

pub trait Packet: Send + Serialize + Sync + 'static { }
