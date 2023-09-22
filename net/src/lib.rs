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
struct TcpListenerComponent(TcpListener);

#[derive(Component)]
struct TcpStreamComponent(TcpStream);

#[derive(Serialize, Deserialize)]
pub enum Packet {
    MyPacket(String)
}
