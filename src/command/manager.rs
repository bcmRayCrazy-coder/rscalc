use std::{collections::HashMap, sync::Arc};

use egui::mutex::RwLock;
use strum::Display;

use crate::command::command::Command;

#[derive(Debug, Display, Clone, PartialEq, Eq)]
pub enum CommandId {}

pub struct CommandManager {
    pub commands:Arc<RwLock<HashMap<CommandId,Box<dyn Command>>>>
}

impl CommandManager {
    pub fn new() -> Self {
        let commands = HashMap::new();
        Self {
            commands:Arc::new(RwLock::new(commands))
        }
    }
}
