use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use bevy::prelude::*;
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
        app.add_systems(Update, (send, recieve));
    }
}

fn send(
    input_keycode: Res<Input<KeyCode>>,
    mut connect_event: EventWriter<net::ConnectEvent>,
    mut disconnect_event: EventWriter<net::DisconnectEvent>,
    mut send_packet_to_server_event: EventWriter<net::SendPacketToServerEvent>
) {
    if input_keycode.just_pressed(KeyCode::Q) {
        connect_event.send(net::ConnectEvent::new(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7777)));
    }

    if input_keycode.just_pressed(KeyCode::W) {
        disconnect_event.send(net::DisconnectEvent);
    }

    if input_keycode.just_pressed(KeyCode::E) {
        send_packet_to_server_event.send(net::SendPacketToServerEvent);
    }
}

fn recieve(
    mut server_unbound_events: EventReader<net::ServerUnboundEvent>,
    mut recieved_packet_from_server_events: EventReader<net::RecievedPacketFromServerEvent>
) {
    for _ in server_unbound_events.iter() {
        println!("Server unbound.");
    }

    for recieved_packet_from_server_event in recieved_packet_from_server_events.iter() {
        match &recieved_packet_from_server_event.packet {
            net::Packet::MyPacket(string) => println!("{}", &string)
        }
    }
}
