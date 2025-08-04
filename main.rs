use pnet::datalink::{self, NetworkInterface};
use pnet::packet::{Packet};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;

fn main() {

    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
        .filter(|iface| !iface.is_loopback() && iface.is_up())
        .next()
        .expect("no suitable interface found");

    println!("Listening on {}", interface.name);

    let (_, mut rx) = datalink::channel(&interface, Default::default())
        .expect("failed to create datalink channel");

    loop {
        match rx.next() {
            Ok(frame) => {
                let ethernet = EthernetPacket::new(frame).unwrap();

                match ethernet.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        let header = Ipv4Packet::new(ethernet.payload());
                        if let Some(ipv4) = header {
                            println!("IPv4 packet: {} -> {}", ipv4.get_source(), ipv4.get_destination());

                            match ipv4.get_next_level_protocol() {
                                IpNextHeaderProtocols::Tcp => {
                                    let tcp = TcpPacket::new(ipv4.payload());
                                    if let Some(tcp_packet) = tcp {
                                        println!("TCP Packet: {}:{} -> {}:{}",
                                            ipv4.get_source(), tcp_packet.get_source(),
                                            ipv4.get_destination(), tcp_packet.get_destination());
                                    }
                                }
                                _ => {
                                    println!("Other IPv4 protocol: {:?}", ipv4.get_next_level_protocol());
                                }
                            }
                        }
                    }
                    _ => {
                        println!("other Ethernet type: {:?}", ethernet.get_ethertype());
                    }
                }
            }
            Err(e) => {
                eprintln!("an error occurred while reading: {}", e);
            }
        }
    }
}
