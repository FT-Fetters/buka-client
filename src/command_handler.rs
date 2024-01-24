#[derive(Debug)]
pub struct CommandHandler {}

impl CommandHandler {
    pub fn new() -> CommandHandler {
        CommandHandler {}
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
            
        } else if show_type == "bucket" {
            
        } else if show_type == "box" {
            
        }else {
            println!("Unknown show tyep");
        }
    }
}
