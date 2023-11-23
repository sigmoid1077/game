use bevy::{
    app::{App, AppExit, Plugin, Startup, Update},
    ecs::event::EventReader,
    input::{keyboard::KeyboardInput, ButtonState},
    DefaultPlugins
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use serde::{Serialize, Deserialize};
use std::{net::ToSocketAddrs, marker::PhantomData};
use util::net;

#[derive(Serialize, Deserialize)]
enum Packet {
    MyPacket(String),
}

impl net::SendingPacket for Packet {
    fn serialize_packet(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

impl net::RecievingPacket for Packet {
    fn deserialize_packet(buffer: &[u8]) -> Self {
        bincode::deserialize(buffer).unwrap()
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            net::ClientPlugin::<Packet, Packet>(PhantomData, PhantomData),
            TestPlugin,
            WorldInspectorPlugin::new()
        ))
        .run();
}

struct TestPlugin;
impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(Update, update);
    }
}

fn startup(mut client: net::ClientSystemParam<Packet, Packet>) {
    let socket_addr = std::env::args()
        .collect::<Vec<_>>()
        .last()
        .unwrap()
        .to_socket_addrs()
        .unwrap()
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .to_owned();
    
    client.connect(socket_addr);
}

fn update(
    mut app_exit_events: EventReader<AppExit>,
    mut client: net::ClientSystemParam<Packet, Packet>,
    mut keyboard_input_events: EventReader<KeyboardInput>
) {
    for _app_exit_event in app_exit_events.read() {
        client.disconnect();
    }
    
    for recieved_packet in client.recieved_packets() {
        match &recieved_packet.0 {
            Packet::MyPacket(string) => println!("{}", string.as_str())
        }
    }
    
    for keyboard_input_event in keyboard_input_events.read() {
        if keyboard_input_event.state == ButtonState::Pressed {
            let packet = Packet::MyPacket(String::from("Packet from client"));
            client.send_packet(packet);
        }
    }
}
