use teloxide::types::Message;

use super::Command;

pub(super) fn is_valid_command(inp: &Message) -> Option<Command> {
    if let Some(msg) = inp.text() {
        let msg = msg.trim().to_lowercase();
        for cmd in vec![Command::New, Command::View, Command::Start, Command::Help] {
            if msg == cmd.to_string() || msg == cmd.to_string()[1..] {
                return Some(cmd);
            }
        }
    }
    None
}
pub(super) fn is_valid_command_exact(inp: Message) -> Option<Command> {
    if let Some(msg) = inp.text() {
        let msg = msg.trim().to_lowercase();
        for cmd in vec![Command::New, Command::View, Command::Start, Command::Help] {
            if msg == cmd.to_string() {
                return Some(cmd);
            }
        }
    }
    None
}
pub(super) fn match_command(msg: &Message, command: Command) -> bool {
    if let Some(text) = msg.text() {
        if text.trim().to_lowercase() == command.to_string()
            || text.trim().to_lowercase() == command.to_string()[1..]
        {
            return true;
        }
    }
    false
}
