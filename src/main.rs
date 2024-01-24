mod buka_client;
mod command_handler;
use buka_client::BukaClient;


fn main() {
    let mut client = BukaClient::new();
    client.version_info();
    client.do_loop();
}
