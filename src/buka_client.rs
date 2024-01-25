use ansi_term::Colour;
use std::io::{self, Write};

use crate::command_handler::CommandHandler;
use crate::request_sender::RequestSender;

#[derive(Debug)]
pub struct BukaClient {
    terminate: bool,
    connected: bool,
    cmd_handler: Option<CommandHandler>,
}

impl BukaClient {

    pub fn new() -> BukaClient {
        BukaClient {
            terminate: false,
            connected: false,
            cmd_handler: None,
        }
    }

    pub fn version_info(&self) {
        println!("buka client 1.0.0");
    }

    pub fn do_loop(&mut self) {
        loop {
            if self.terminate {
                break;
            }
            self.print_head();
            // 刷新缓冲，解决因为行缓冲导致print无法显示的问题
            if io::stdout().flush().is_err() {
                print!("Flush error");
                return;
            }
            let mut input = String::new();
            let len = std::io::stdin().read_line(&mut input).unwrap();
            if len > 0 {
                self.handle_input(input);
            }
        }
    }

    fn handle_input(&mut self, input: String) {
        // println!("input: {}", input);
        let _input = input.trim();
        let sp: Vec<String> = _input.split(" ").map(|s: &str| s.to_string()).collect();
        if sp[0] == "connect" {
            if self.connected {
                println!("Connected");
                return;
            }
            self.do_connect(sp);
            return;
        }
        if !self.connected {
            println!("Not connect yet, use connect [ip] [port] [key] to connect");
            return;
        }
        let selt_cmd_handler = self.cmd_handler.take();
        if let Some(mut handler) = selt_cmd_handler {
            handler.handle(sp);
            self.cmd_handler = Option::Some(handler);
        }
    }

    fn do_connect(&mut self, cmd: Vec<String>) {
        let cmd_len = cmd.len();
        if cmd_len < 3 {
            println!("Do you want to connect? try like this: connect 127.0.0.1 8771 123456");
            return;
        }
        let ip = &cmd[1];
        let port: i32 = cmd[2].parse().unwrap();
        let mut key: &String = &String::from("");
        if cmd_len > 3 {
            key = &cmd[3];
        }
        println!("connecting....");
        let req_sender = RequestSender::new(ip, port, key);
        if req_sender.connect() {
            self.connected = true;
            self.cmd_handler = Some(CommandHandler::new(req_sender));
            println!("{}", Colour::Green.paint("Connected"));
        } else {
            println!("Connect fail, may auth key is wroing");
        }
    }

    fn print_head(&mut self) {
        let status_color: ansi_term::ANSIGenericString<'_, str>;
        if self.connected {
            status_color = Colour::Green.paint("~");
        } else {
            status_color = Colour::Red.paint("~");
        }
        print!("({})>> ", status_color);
    }
}
