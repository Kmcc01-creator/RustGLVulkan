// src/ui/integration.rs
use std::sync::mpsc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum UIMessage {
    MemoryUpdate(MemoryStats),
    GraphicsUpdate(GraphicsStats),
    DebugCommand(DebugCommand),
}

pub struct UIBridge {
    tx: mpsc::Sender<UIMessage>,
    rx: mpsc::Receiver<UIMessage>,
    web_context: WebContext,
}

impl UIBridge {
    pub fn send_to_ui(&self, msg: UIMessage) {
        self.tx.send(msg).unwrap();
    }

    pub fn handle_ui_messages(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                UIMessage::DebugCommand(cmd) => {
                    // Handle debug commands from UI
                },
                _ => {}
            }
        }
    }
}