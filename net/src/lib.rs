use std::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};
use bevy::prelude::*;

pub mod client;
pub mod server;

pub use client::{
    ClientPlugin,
    ConnectEvent,
    ServerUnboundEvent,
    DisconnectEvent,
    SendPacketToServerEvent,
    RecievedPacketFromServerEvent,
};

pub use server::{
    ServerPlugin,
    BindEvent,
    UnbindEvent,
    ClientConnectedEvent,
    ClientDisconnectedEvent,
    SendPacketToClientEvent,
    SendPacketToAllClientsEvent,
    RecievedPacketFromClientEvent
};

#[derive(Component)]
struct TcpListenerComponent {
    pub tcp_listener: TcpListener
}

impl TcpListenerComponent {
    fn new(tcp_listener: TcpListener) -> Self {
        Self { tcp_listener }
    }
}

#[derive(Component)]
struct TcpStreamComponent {
    pub tcp_stream: TcpStream
}

impl TcpStreamComponent {
    fn new(tcp_stream: TcpStream) -> Self {
        Self { tcp_stream }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Packet {
    MyPacket(String)
}
