// src/ui/bridge.rs
use std::sync::mpsc::{self, Sender, Receiver};
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum UIMessage {
    // Memory-related messages
    MemoryStats(MemoryStatsUpdate),
    MemoryAlert(MemoryAlert),
    
    // Graphics-related messages
    FrameStats(FrameStatsUpdate),
    GraphicsMemoryUpdate(GraphicsMemoryStats),
    
    // Debug commands
    Command(DebugCommand),
    
    // UI state updates
    UpdateLayout(LayoutUpdate),
    Refresh,
}

#[derive(Debug, Clone)]
pub struct MemoryStatsUpdate {
    total_allocated: usize,
    peak_usage: usize,
    block_usage: Vec<f32>,
    fragmentation: f32,
    timestamp: std::time::SystemTime,
}

pub struct UIBridge {
    to_ui: Sender<UIMessage>,
    from_ui: Receiver<UIMessage>,
    state: Arc<RwLock<UIState>>,
}

#[derive(Default)]
pub struct UIState {
    visible: bool,
    layout: LayoutConfig,
    active_components: Vec<ComponentId>,
    last_update: std::time::SystemTime,
}

impl UIBridge {
    pub fn new() -> (Self, UISender) {
        let (to_ui_tx, to_ui_rx) = mpsc::channel();
        let (from_ui_tx, from_ui_rx) = mpsc::channel();
        let state = Arc::new(RwLock::new(UIState::default()));
        
        let bridge = UIBridge {
            to_ui: to_ui_tx,
            from_ui: from_ui_rx,
            state: Arc::clone(&state),
        };
        
        let sender = UISender {
            sender: from_ui_tx,
            state: Arc::clone(&state),
        };
        
        (bridge, sender)
    }
    
    pub fn process_messages(&mut self) {
        while let Ok(message) = self.from_ui.try_recv() {
            self.handle_message(message);
        }
    }
    
    fn handle_message(&mut self, message: UIMessage) {
        match message {
            UIMessage::Command(cmd) => self.handle_command(cmd),
            UIMessage::UpdateLayout(layout) => self.update_layout(layout),
            _ => {}
        }
    }
}