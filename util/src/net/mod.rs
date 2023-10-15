use bevy::ecs::component::Component;
use serde::{Deserialize, Serialize};
use std::net::{TcpListener, TcpStream};

pub mod client;
pub mod server;

pub use client::{
    ClientPlugin, ConnectEvent, DisconnectEvent, RecievedPacketFromServerEvent,
    SendPacketToServerEvent, ServerUnboundEvent,
};

pub use server::{
    BindEvent, ClientConnectedEvent, ClientDisconnectedEvent, RecievedPacketFromClientEvent,
    SendPacketToAllClientsEvent, SendPacketToClientEvent, ServerPlugin, UnbindEvent,
};

#[derive(Component)]
struct TcpListenerComponent {
    pub tcp_listener: TcpListener,
}

impl TcpListenerComponent {
    fn new(tcp_listener: TcpListener) -> Self {
        Self { tcp_listener }
    }
}

#[derive(Component)]
struct TcpStreamComponent {
    pub tcp_stream: TcpStream,
}

impl TcpStreamComponent {
    fn new(tcp_stream: TcpStream) -> Self {
        Self { tcp_stream }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Packet {
    MyPacket(String),
}
