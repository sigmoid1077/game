use std::marker::PhantomData;

use bevy::{
    app::{App, AppExit, Plugin, Startup, Update},
    ecs::event::{EventReader, EventWriter},
    input::{keyboard::KeyboardInput, ButtonState},
    DefaultPlugins
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use serde::{Deserialize, Serialize};
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
            net::ServerPlugin::<Packet, Packet>(PhantomData, PhantomData),
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

fn startup(mut bind_event: EventWriter<net::server::event::write::BindEvent>) {
    bind_event.send(net::server::event::write::BindEvent(std::env::args().collect::<Vec<_>>().last().unwrap().parse().unwrap()));
}

fn update(
    mut app_exit_events: EventReader<AppExit>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut unbind_event: EventWriter<net::server::event::write::UnbindEvent>,
    mut recieved_packet_from_client_events: EventReader<net::server::event::read::RecievedPacketFromClientEvent<Packet>>,
    mut send_packet_to_all_clients_event: EventWriter<net::server::event::write::SendPacketToAllClients<Packet>>
) {
    for _ in app_exit_events.read() {
        unbind_event.send(net::server::event::write::UnbindEvent);
    }

    for recieved_packet_from_client_event in recieved_packet_from_client_events.read() {
        match &recieved_packet_from_client_event.0 {
            Packet::MyPacket(string) => println!("{}", &string)
        }
    }

    for keyboard_input_event in keyboard_input_events.read() {
        if keyboard_input_event.state == ButtonState::Pressed {
            send_packet_to_all_clients_event.send(net::server::event::write::SendPacketToAllClients(Packet::MyPacket(format!("{:?}", keyboard_input_event.key_code.unwrap()))));
        }
    }
}
