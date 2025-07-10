mod cli;
mod core;

use std::net::TcpStream;
use std::io::Read;

fn receive_network_data() -> anyhow::Result<String> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let mut buffer = [0; 1024];
    //SOURCE
    let bytes_read = stream.read(&mut buffer)?;
    let received_data = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
    Ok(received_data)
}

fn main() -> anyhow::Result<()> {
    // print a cool banner!
    cli::interface::banner();

    // receive network data and process it
    let network_data = receive_network_data()?;
    cli::file_processor::process_network_data(network_data)?;

    // engine takes off!
    core::engine::init()
}
