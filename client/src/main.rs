use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use bevy::{
    app::{App, AppExit, Plugin, Update, Startup},
    DefaultPlugins,
    ecs::event::{EventReader, EventWriter},
    input::{keyboard::KeyboardInput, ButtonState}
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            net::ClientPlugin, 
            DefaultPlugins, 
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

fn startup(
    mut connect_event: EventWriter<net::ConnectEvent>,
) {
    connect_event.send(net::ConnectEvent::new(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7777)));
}

fn update(
    mut app_exit_events: EventReader<AppExit>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut disconnect_event: EventWriter<net::DisconnectEvent>,
    mut recieved_packet_from_server_events: EventReader<net::RecievedPacketFromServerEvent>,
    mut send_packet_to_server_event: EventWriter<net::SendPacketToServerEvent>
) {
    for _ in app_exit_events.iter() {
        disconnect_event.send(net::DisconnectEvent);
    }

    for recieved_packet_from_server_event in recieved_packet_from_server_events.iter() {
        match &recieved_packet_from_server_event.packet {
            net::Packet::MyPacket(string) => println!("{}", &string)
        }
    }

    for keyboard_input_event in keyboard_input_events.iter() {
        if keyboard_input_event.state == ButtonState::Pressed {
            send_packet_to_server_event.send(net::SendPacketToServerEvent::new(net::Packet::MyPacket(format!("{:?}", keyboard_input_event.key_code.unwrap()))));
        }
    }
}
