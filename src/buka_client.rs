use ansi_term::Colour;
use std::io::{self, Write};

use crate::command_handler::CommandHandler;

#[derive(Debug)]
pub struct BukaClient {
    terminate: bool,
    connected: bool,
    cmd_handler: CommandHandler,
}

impl BukaClient {
    pub fn new() -> BukaClient {
        BukaClient {
            terminate: false,
            connected: false,
            cmd_handler: CommandHandler::new(),
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
            self.do_connect();
            return;
        }
        if !self.connected {
            println!("Not connect yet, use connect [ip] [port] to connect");
            return;
        }
        self.cmd_handler.handle(sp);
    }

    fn do_connect(&mut self) {
        self.connected = true;
        println!("connecting....")
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
