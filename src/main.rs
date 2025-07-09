mod cli;
mod core;

use std::net::{TcpStream, UdpSocket};
use std::io::Read;

fn receive_network_data() -> anyhow::Result<String> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    //SOURCE
    let received_data = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
    Ok(received_data)
}

fn receive_udp_data() -> anyhow::Result<String> {
    let socket = UdpSocket::bind("127.0.0.1:8081")?;
    let mut buffer = [0; 1024];
    let bytes_read = socket.recv(&mut buffer)?;
    //SOURCE
    let received_data = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
    Ok(received_data)
}

fn main() -> anyhow::Result<()> {
    // print a cool banner!
    cli::interface::banner();

    // receive network data and process it
    let network_data = receive_network_data()?;
    cli::file_processor::process_network_data(network_data)?;

    // receive UDP data and process redirect
    let udp_data = receive_udp_data()?;
    cli::redirect_handler::process_redirect_data(udp_data)?;

    // engine takes off!
    core::engine::init()
}
