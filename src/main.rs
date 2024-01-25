mod buka_client;
mod command_handler;
mod request_sender;
use buka_client::BukaClient;

fn main() {
    let mut client = BukaClient::new();
    client.version_info();
    client.do_loop();
}