use crate::command::manager::CommandId;

pub enum CommandUiList {
    Icon(CommandId),
    Block(CommandId),
    Multi(Vec<CommandId>),
    List([Option<CommandId>; 3]),
}

impl CommandUiList {}
