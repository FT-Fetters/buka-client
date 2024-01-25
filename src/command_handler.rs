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
        }else if cmd[0] == "exit" {
            exit(-1);
        }else {
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
        } else if show_type == "box" {
        } else {
            println!("Unknown show type");
        }
    }
}
