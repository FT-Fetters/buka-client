use std::io::{self, Write};
use std::process::exit;

use crate::request_sender::RequestSender;

#[derive(Debug)]
pub struct CommandHandler {
    req_sender: RequestSender,
}

impl CommandHandler {
    pub fn new(req_sender: RequestSender) -> CommandHandler {
        let cmd_hdl = CommandHandler {
            req_sender: req_sender,
        };
        return cmd_hdl;
    }

    /**
     * 主处理函数
     */
    pub fn handle(&mut self, cmd: Vec<String>) {
        let cmd_len = cmd.len();
        if cmd_len == 0 {
            return;
        }
        if cmd[0] == "show" {
            // 处理 show 命令
            self.handle_show(cmd);
        } else if cmd[0] == "bput" {
            self.handle_bucket_put(cmd);
        } else if cmd[0] == "create" {
            self.handle_create(cmd);
        } else if cmd[0] == "exit" {
            exit(-1);
        } else {
            println!("Unknown command, use [help]?");
        }
    }

    fn handle_show(&self, cmd: Vec<String>) {
        let cmd_len = cmd.len();
        if cmd_len < 2 {
            println!("Want to use show? use it like this: show [type]");
            println!("[type] =: (repo, bucket, box)");
            return;
        }
        let show_type: &String = &cmd[1];
        if show_type == "repo" {
            let repo_vec: Vec<String> = self.req_sender.show_repo();
            for ele in repo_vec {
                println!("- {}", ele);
            }
        } else if show_type == "bucket" {
            if cmd_len < 3 {
                println!("Need repository name");
                println!("Trye like this: show bucket [repo]");
                return;
            }
            let rep_name = &cmd[2];
            let bucket_vec = self.req_sender.show_bucket(rep_name.to_string());
            for ele in bucket_vec {
                println!("- {}", ele);
            }
        } else if show_type == "box" {
        } else {
            println!("Unknown show type");
        }
    }

    fn handle_bucket_put(&self, cmd: Vec<String>) {
        let cmd_len = cmd.len();
        if cmd_len < 3 {
            println!("Error for bput. use like this: bput [repo] [bucket]");
            return;
        }
        let repo = cmd[1].to_string();
        let bucket = cmd[2].to_string();
        print!(" field > ");
        if io::stdout().flush().is_err() {
            print!("Flush error");
            return;
        }
        let mut input = String::new();
        let len = std::io::stdin().read_line(&mut input).unwrap();
        if len == 0 {
            println!("Bucket put exit");
            return;
        }
        input = input.trim().to_string();
    }

    fn handle_create(&self, cmd: Vec<String>) {
        let cmd_len = cmd.len();
        if cmd_len < 2 {
            println!("Wrong create command usage");
            println!("- create repo [name]");
            println!("- create bucket [repo] [name]");
            return;
        }
        let create_type = cmd[1].to_string();
        if create_type == "repo" {
            self.handle_create_repo(cmd);
        } else if create_type == "bucket" {
            self.handle_create_bucket(cmd);
        } else {
            println!("Wrong create type, should be repo or bucket");
        }
    }

    fn handle_create_repo(&self, cmd: Vec<String>) {
        let cmd_len = cmd.len();
        if cmd_len < 3 {
            println!("Need repository name");
            return;
        }
        let repo_name = cmd[2].to_string();
        self.req_sender.create_repo(repo_name);
    }

    fn handle_create_bucket(&self, cmd: Vec<String>) {}
}
